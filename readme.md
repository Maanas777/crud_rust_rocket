# CRUD API with Rust and Rocket

This project is a simple CRUD (Create, Read, Update, Delete) API built using Rust and Rocket, with MongoDB as the database. The API allows you to manage user data.

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/learn/get-started)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [MongoDB](https://docs.mongodb.com/manual/installation/)
- [Postman](https://www.postman.com/downloads/) (optional, for testing)

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Maanas777/crud_rust_rocket
   cd crud_rust_rocket

 2. **Install Rust dependencies**
   ```bash
    cargo build

## Running the Application
     
  **Run the Rocket Server**
    ``bash
      cargo run
 


## API Endpoints


**Create a User**

``bash
curl -X POST http://localhost:8000/create_user -H "Content-Type: application/json" -d '{
    "name": "Alice",
    "location": "Los Angeles",
    "description": "Designer"
}


**Read a User**

``bash
 curl -X GET http://localhost:8000/get_user/<id>


**Update a user**

``bash
curl -X PUT http://localhost:8000/update_user/<id> -H "Content-Type: application/json" -d '{
    "name": "Jane Doe",
    "location": "San Francisco",
    "description": "Product Manager"
}'

   
   **Delete a User**
   curl -X DELETE http://localhost:8000/delete_user/<id>



   
   
