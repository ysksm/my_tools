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

// Load jira.db file
const response = await fetch('/poc_app1/output/jira.db');
const arrayBuffer = await response.arrayBuffer();
await db.registerFileBuffer('jira.db', new Uint8Array(arrayBuffer));

// Attach the database
await c.query(`ATTACH 'jira.db' AS jira`);

// Function to get all projects
async function getAllProjects() {
  const result = await c.query(`SELECT * FROM jira.projects ORDER BY id`);
  return result.toArray();
}

// Get initial projects
const initialProjects = await getAllProjects();
console.log('Initial projects:', initialProjects);

await c.close();

function App() {
  const [projects, setProjects] = useState(initialProjects)

  return (
    <>
      <div className="project-container">
        <h1>Jira Projects</h1>
        <div className="project-list">
          {projects.map((project: any) => (
            <div key={project.id} className="project-item">
              <h3>{project.name}</h3>
              <div className="project-details">
                <span className="project-key">Key: {project.key}</span>
                <span className="project-type">Type: {project.type}</span>
                <span className="project-style">Style: {project.style}</span>
                <span className="project-id">ID: {project.id}</span>
              </div>
            </div>
          ))}
        </div>
      </div>
    </>
  )
}

export default App
