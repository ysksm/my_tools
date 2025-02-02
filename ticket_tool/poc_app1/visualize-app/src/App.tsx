import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

import * as duckdb from '@duckdb/duckdb-wasm';
import duckdb_wasm from '@duckdb/duckdb-wasm/dist/duckdb-mvp.wasm?url';
import mvp_worker from '@duckdb/duckdb-wasm/dist/duckdb-browser-mvp.worker.js?url';
import duckdb_wasm_eh from '@duckdb/duckdb-wasm/dist/duckdb-eh.wasm?url';
import eh_worker from '@duckdb/duckdb-wasm/dist/duckdb-browser-eh.worker.js?url';

const MANUAL_BUNDLES: duckdb.DuckDBBundles = {
    mvp: {
        mainModule: duckdb_wasm,
        mainWorker: mvp_worker,
    },
    eh: {
        mainModule: duckdb_wasm_eh,
        mainWorker: eh_worker,
    },
};
// Select a bundle based on browser checks
const bundle = await duckdb.selectBundle(MANUAL_BUNDLES);
// Instantiate the asynchronus version of DuckDB-wasm
const worker = new Worker(bundle.mainWorker!);
const logger = new duckdb.ConsoleLogger();
const db = new duckdb.AsyncDuckDB(logger, worker);
await db.instantiate(bundle.mainModule, bundle.pthreadWorker);

const c = await db.connect();



// Create todo table
await c.query(`
  CREATE TABLE IF NOT EXISTS todos (
    id INTEGER PRIMARY KEY,
    title VARCHAR,
    completed BOOLEAN,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
  )
`);

// Insert sample data
await c.query(`
  INSERT INTO todos (id, title, completed) VALUES
  (1, 'Buy groceries', false),
  (2, 'Complete project report', true),
  (3, 'Exercise for 30 minutes', false),
  (4, 'Read a book', false),
  (5, 'Clean the house', true)
`);

// Function to get all todos
async function getAllTodos() {
  const result = await c.query(`SELECT * FROM todos ORDER BY id`);
  return result.toArray();
}

// Get initial todos
const initialTodos = await getAllTodos();
console.log('Initial todos:', initialTodos);

await c.close();


function App() {
  const [todos, setTodos] = useState(initialTodos)

  return (
    <>
      <div className="todo-container">
        <h1>Todo List</h1>
        <div className="todo-list">
          {todos.map((todo: any) => (
            <div key={todo.id} className="todo-item">
              <input
                type="checkbox"
                checked={todo.completed}
                readOnly
              />
              <span style={{ textDecoration: todo.completed ? 'line-through' : 'none' }}>
                {todo.title}
              </span>
              <span className="todo-date">
                {new Date(todo.created_at).toLocaleString()}
              </span>
            </div>
          ))}
        </div>
      </div>
    </>
  )
}

export default App
