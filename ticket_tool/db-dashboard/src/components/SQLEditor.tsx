import React, { useState } from 'react';
import Editor from '@monaco-editor/react';
import { useMutation } from '@tanstack/react-query';
import { api, QueryResponse } from '../api/client';
import './SQLEditor.css';

interface SQLEditorProps {
  onQueryResult: (result: QueryResponse) => void;
}

export const SQLEditor: React.FC<SQLEditorProps> = ({ onQueryResult }) => {
  const [sql, setSql] = useState('SELECT * FROM your_table LIMIT 10;');
  
  const queryMutation = useMutation({
    mutationFn: (sql: string) => api.executeQuery(sql),
    onSuccess: (response) => {
      onQueryResult(response.data);
    },
  });

  const handleExecute = () => {
    if (sql.trim()) {
      queryMutation.mutate(sql);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
      handleExecute();
    }
  };

  return (
    <div className="sql-editor" onKeyDown={handleKeyDown}>
      <div className="editor-header">
        <h3>SQL Query Editor</h3>
        <button 
          onClick={handleExecute}
          disabled={queryMutation.isPending}
          className="execute-button"
        >
          {queryMutation.isPending ? 'Executing...' : 'Execute (Ctrl+Enter)'}
        </button>
      </div>
      
      {queryMutation.isError && (
        <div className="error-message">
          Error: {queryMutation.error instanceof Error ? queryMutation.error.message : 'Unknown error'}
        </div>
      )}
      
      <div className="editor-container">
        <Editor
          height="300px"
          defaultLanguage="sql"
          theme="vs-light"
          value={sql}
          onChange={(value) => setSql(value || '')}
          options={{
            minimap: { enabled: false },
            fontSize: 14,
            lineNumbers: 'on',
            scrollBeyondLastLine: false,
            automaticLayout: true,
            wordWrap: 'on',
          }}
        />
      </div>
    </div>
  );
};