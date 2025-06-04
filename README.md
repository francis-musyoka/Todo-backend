# Todo List Backend (Rust)

A high-performance and secure backend API for a Todo List application, developed using **Rust** and the **Actix-web** framework. This backend provides RESTful endpoints for user authentication, profile management, and full CRUD operations on todos.

---

## Features

- **User Authentication**  
  - Register new users  
  - Login with secure token-based authentication  

- **Profile Management**  
  - View and update user profile information  

- **Todo Management**  
  - Create, read, update, and delete todos  
  - Associate todos with authenticated users  

- **Robust and Efficient**  
  - Built with Rust for memory safety and high performance  
  - Uses Actix-web for fast asynchronous web server capabilities  

---

## Tech Stack

- [Rust](https://www.rust-lang.org/)  
- [Actix-web](https://actix.rs/)  
- [Diesel](https://diesel.rs/) or [SQLx](https://github.com/launchbadge/sqlx) (if applicable) for database ORM  
- JWT or similar for authentication tokens  

---

## Getting Started  

1. Clone the repository
- git clone https://github.com/francis-musyoka/Todo-backend.git
- cd Todo-backend
- cargo watch -x run

### Clone the Frontend repository 
-  The frontend is implemented in Next.js + TypeScriptand lives in a separate repository.
-  git clone https://github.com/francis-musyoka/Todo-frontend.git
 - cd Todo-frontend
 - npm run dev

