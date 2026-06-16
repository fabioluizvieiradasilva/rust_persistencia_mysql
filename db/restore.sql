CREATE DATABASE IF NOT EXISTS client_rust_db;
USE client_rust_db;

CREATE TABLE IF NOT EXISTS client (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(150) NOT NULL,
    email VARCHAR(150) NOT NULL UNIQUE
);

INSERT INTO client (name, email) VALUES
('John Doe', 'john@email.com'),
('Jane Smith', 'jane@email.com'),
('Alice Johnson', 'alice@email.com'),
('Bob Brown', 'bob@email.com');