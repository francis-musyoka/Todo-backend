use actix_web::{
    Error, HttpResponse, Responder, error, post, put,
    web::{self, Data},
};
use chrono::Utc;

use crate::helpers::{
    generate_id, hash_password, is_email_valid, is_not_empty, verify_and_match_password,
    verify_password,
};
use crate::models::{
    CreateUser, CreateUserResponse, LogInUser, UpdateUserInfo, UpdateUserPassword, User,
    UserLoginResponse, UserState,
};

#[post("/create/user")]

pub async fn create_user(
    user_fields: web::Json<CreateUser>,
    data: Data<UserState>,
) -> Result<impl Responder, Error> {
    // Take ownership of the inner CreateUser struct immediately for easier access
    let user_data = user_fields.into_inner();

    // struct destructuring
    let CreateUser {
        first_name,
        last_name,
        email,
        password,
        confirm_password,
    } = user_data;

    // Validate last_name &&  last_name

    if !is_not_empty(&first_name) {
        return Ok(HttpResponse::BadRequest().body("Firstname cannot be empty"));
    }

    if !is_not_empty(&last_name) {
        return Ok(HttpResponse::BadRequest().body("Lastname cannot be empty"));
    }

    // Validate Email

    if !is_email_valid(&email) {
        return Ok(HttpResponse::BadRequest().body("Invalid email format"));
    }

    // Verify Passwords and match

    match verify_and_match_password(&password, &confirm_password) {
        Ok(_) => println!("password good"),
        Err(mgs) => {
            return Ok(HttpResponse::BadRequest().body(mgs));
        }
    }

    // let hashed_password = web::block(move || hash_password(&password)).await??;

    let hashed_password = web::block(move || hash_password(&password))
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let mut users = data
        .user_list
        .lock()
        .map_err(|_| error::ErrorInternalServerError("Could not unlock Users"))?;

    // Check if email exist
    if users.iter_mut().any(|user| user.email == email) {
        return Ok(HttpResponse::Conflict().body("Email already used"));
    }

    let id = generate_id();

    let new_user = User {
        id,
        first_name,
        last_name,
        email,
        password: hashed_password,
        created_at: Utc::now(),
        update_at: Utc::now(),
    };

    users.push(new_user.clone());

    Ok(HttpResponse::Created().json(CreateUserResponse {
        id: new_user.id,
        first_name: new_user.first_name,
        last_name: new_user.last_name,
        email: new_user.email,
        created_at: new_user.created_at,
    }))
}

// HANDLE LOG IN

#[post("/login")]
pub async fn log_in_user(
    user_fields: web::Json<LogInUser>,
    data: Data<UserState>,
) -> Result<impl Responder, Error> {
    let user_data = user_fields.into_inner();
    let LogInUser { email, password } = user_data;

    // Clone user with associated email
    let found_user = {
        // get users
        let users = data
            .user_list
            .lock()
            .map_err(|_| actix_web::error::ErrorInternalServerError("Could not unlock Users"))?;

        // Find the user by email and clone
        if let Some(user) = users.iter().find(|u| u.email == email) {
            user.clone()
        } else {
            return Err(actix_web::error::ErrorUnauthorized(
                "Invalid email or password",
            )); // Return if user Not Found
        }
    }; // lock dropped here because `users` goes out of scope

    let user_password_clone = found_user.password.clone(); // Clone the user's stored password

    let is_password_valid = web::block(move || verify_password(&password, &user_password_clone))
        .await
        .map_err(|e| error::ErrorInternalServerError(format!("Blocking operation failed: {}", e)))?
        .map_err(|e| {
            error::ErrorInternalServerError(format!("Password verification error: {}", e))
        })?;

    if is_password_valid {
        Ok(HttpResponse::Ok().json(UserLoginResponse {
            id: found_user.id.clone(),
            first_name: found_user.first_name.clone(),
            last_name: found_user.last_name.clone(),
            email: found_user.email.clone(),
        }))
    } else {
        Ok(HttpResponse::Unauthorized().body("Invalid email or password"))
    }
}

// HANDLE UPDATE USER INFO EMAIL, FIRST NAME AND LAST NAME
#[put("/update/user/{id}")]
pub async fn update_user_info(
    path: web::Path<String>,
    user_fields: web::Json<UpdateUserInfo>,
    data: Data<UserState>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    let user_data = user_fields.into_inner();

    let mut users = data
        .user_list
        .lock()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Could not unlock Users"))?;

    // FIND USER INDEX
    let user_index = users.iter().position(|u| u.id == id);

    if let Some(index) = user_index {
        // IF EMAIL VALIDED AND CHECK UNIQUENESS
        if let Some(ref new_email) = user_data.email {
            //Valided Email
            if !is_email_valid(new_email) {
                return Ok(HttpResponse::BadRequest().body("Invalid email format."));
            }

            // Check if this new email exists for *any other* user.
            if users
                .iter() // immutable
                .enumerate()
                .any(|(i, u)| i != index && u.email == *new_email)
            {
                return Ok(HttpResponse::Conflict().body("Email already in use by another user."));
            }
        }

        // ----mutable----

        // GET USER
        let user = &mut users[index];

        // Update first_name if provided and not empty
        if let Some(new_first_name) = user_data.first_name {
            if !is_not_empty(&new_first_name) {
                return Ok(HttpResponse::BadRequest().body("First name cannot be empty."));
            }

            user.first_name = new_first_name;
        }

        // Update last_name if provided and not empty
        if let Some(new_last_name) = user_data.last_name {
            if !is_not_empty(&new_last_name) {
                return Ok(HttpResponse::BadRequest().body("Last name cannot be empty."));
            }

            user.last_name = new_last_name;
        }

        // Update email if provided and validate
        if let Some(new_email) = user_data.email {
            user.email = new_email;
        }

        // Update the timestamp
        user.update_at = Utc::now();

        // Return the updated user info
        Ok(HttpResponse::Ok().json(UserLoginResponse {
            id: user.id.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
        }))
    } else {
        // User Not Found
        Ok(HttpResponse::NotFound().body("User not found"))
    }
}

// HANDLE UPDATE USER PASSWORD
#[put("/update/user/{id}/changepassword")]
pub async fn update_user_password(
    path: web::Path<String>,
    user_fields: web::Json<UpdateUserPassword>,
    data: Data<UserState>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    let user_data = user_fields.into_inner();

    // struct destructuring
    let UpdateUserPassword {
        current_password,
        new_password,
        confirm_password,
    } = user_data;

    //Find User By ID
    let mut found_user = {
        // Get all users
        let users = data
            .user_list
            .lock()
            .map_err(|_| actix_web::error::ErrorInternalServerError("Could not unlock Users"))?;

        // Find the user by ID and clone
        if let Some(user) = users.iter().find(|u| u.id == id) {
            user.clone()
        } else {
            return Err(actix_web::error::ErrorUnauthorized(
                "Invalid user ID or old password.",
            )); // Return if user Not Found
        }
    };

    // Check if current password and new password are same
    if new_password == current_password {
        return Ok(
            HttpResponse::Unauthorized().body("Old password and new password cannot be same")
        );
    }

    // Verify password and Match it with confirm password

    // OPTION 1:
    verify_and_match_password(&new_password, &confirm_password)
        .map_err(actix_web::error::ErrorBadRequest)?;

    /*
    // OPTION 2;
    match verify_and_match_password(&password, &confirm_password) {
        Ok(_) => println!("password good"),
        Err(mgs) => {
            return Ok(HttpResponse::BadRequest().body(mgs));
        }
    }
    */

    // Match Current Password against stored password

    let user_password_clone = found_user.password.clone();

    let is_password_matching =
        web::block(move || verify_password(&current_password, &user_password_clone))
            .await
            .map_err(|e| {
                error::ErrorInternalServerError(format!("Blocking operation failed: {}", e))
            })?
            .map_err(|e| {
                error::ErrorInternalServerError(format!("Password verification error: {}", e))
            })?;

    if is_password_matching {
        // If current password and new password match hash the password and update
        let hashed_password = web::block(move || hash_password(&new_password))
            .await
            .map_err(|e| {
                error::ErrorInternalServerError(format!("Blocking operation failed: {}", e))
            })?
            .map_err(|e| {
                error::ErrorInternalServerError(format!("Password verification error: {}", e))
            })?;

        // update Password
        found_user.password = hashed_password;

        Ok(HttpResponse::Ok().json("Password updated succefully"))
    } else {
        Ok(HttpResponse::Unauthorized().body("Invalid credentials...p"))
    }
}

//pub async fn reset_password() -> Result<impl Responder, Error> {}
