-- Create sample tables for testing
CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL,
    category VARCHAR,
    price DECIMAL(10, 2),
    stock_quantity INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS customers (
    id INTEGER PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR UNIQUE,
    city VARCHAR,
    country VARCHAR,
    registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS orders (
    id INTEGER PRIMARY KEY,
    customer_id INTEGER REFERENCES customers(id),
    product_id INTEGER REFERENCES products(id),
    quantity INTEGER NOT NULL,
    total_amount DECIMAL(10, 2),
    order_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    status VARCHAR CHECK (status IN ('pending', 'shipped', 'delivered', 'cancelled'))
);

-- Insert sample data
INSERT INTO products (id, name, category, price, stock_quantity) VALUES
(1, 'Laptop Pro 15', 'Electronics', 1299.99, 45),
(2, 'Wireless Mouse', 'Electronics', 29.99, 150),
(3, 'USB-C Hub', 'Electronics', 49.99, 80),
(4, 'Office Chair', 'Furniture', 399.99, 25),
(5, 'Standing Desk', 'Furniture', 599.99, 15),
(6, 'Monitor 27"', 'Electronics', 349.99, 60),
(7, 'Keyboard Mechanical', 'Electronics', 129.99, 90),
(8, 'Desk Lamp LED', 'Furniture', 89.99, 40),
(9, 'Notebook Set', 'Stationery', 19.99, 200),
(10, 'Coffee Maker', 'Appliances', 199.99, 30);

INSERT INTO customers (id, first_name, last_name, email, city, country) VALUES
(1, 'John', 'Doe', 'john.doe@email.com', 'New York', 'USA'),
(2, 'Jane', 'Smith', 'jane.smith@email.com', 'London', 'UK'),
(3, 'Bob', 'Johnson', 'bob.j@email.com', 'Toronto', 'Canada'),
(4, 'Alice', 'Williams', 'alice.w@email.com', 'Sydney', 'Australia'),
(5, 'Charlie', 'Brown', 'charlie.b@email.com', 'Berlin', 'Germany');

INSERT INTO orders (id, customer_id, product_id, quantity, total_amount, status) VALUES
(1, 1, 1, 1, 1299.99, 'delivered'),
(2, 1, 2, 2, 59.98, 'delivered'),
(3, 2, 6, 1, 349.99, 'shipped'),
(4, 3, 4, 1, 399.99, 'pending'),
(5, 4, 7, 1, 129.99, 'delivered'),
(6, 5, 10, 1, 199.99, 'shipped'),
(7, 2, 3, 3, 149.97, 'delivered'),
(8, 3, 5, 1, 599.99, 'cancelled'),
(9, 1, 9, 5, 99.95, 'delivered'),
(10, 4, 8, 2, 179.98, 'pending');