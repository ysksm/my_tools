import { useState } from 'react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { SchemaExplorer } from './components/SchemaExplorer'
import { SQLEditor } from './components/SQLEditor'
import { DataTable } from './components/DataTable'
import { QueryResponse } from './api/client'
import './App.css'

const queryClient = new QueryClient()

function App() {
  const [queryResult, setQueryResult] = useState<QueryResponse | null>(null)

  return (
    <QueryClientProvider client={queryClient}>
      <div className="app">
        <header className="app-header">
          <h1>DuckDB Dashboard</h1>
        </header>
        <div className="app-content">
          <aside className="sidebar">
            <SchemaExplorer />
          </aside>
          <main className="main-content">
            <div className="content-wrapper">
              <SQLEditor onQueryResult={setQueryResult} />
              <div className="results-section">
                <DataTable data={queryResult} />
              </div>
            </div>
          </main>
        </div>
      </div>
    </QueryClientProvider>
  )
}

export default App
