import React, { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { api, TableInfo } from '../api/client';
import './SchemaExplorer.css';

export const SchemaExplorer: React.FC = () => {
  const [expandedTables, setExpandedTables] = useState<Set<string>>(new Set());
  
  const { data, isLoading, error } = useQuery({
    queryKey: ['schema'],
    queryFn: async () => {
      const response = await api.getSchema();
      return response.data;
    },
  });

  const toggleTable = (tableName: string) => {
    setExpandedTables(prev => {
      const newSet = new Set(prev);
      if (newSet.has(tableName)) {
        newSet.delete(tableName);
      } else {
        newSet.add(tableName);
      }
      return newSet;
    });
  };

  if (isLoading) {
    return <div className="schema-explorer">Loading schema...</div>;
  }

  if (error) {
    return (
      <div className="schema-explorer error">
        Error loading schema: {error instanceof Error ? error.message : 'Unknown error'}
      </div>
    );
  }

  return (
    <div className="schema-explorer">
      <h2>Database Schema</h2>
      {data?.tables.length === 0 ? (
        <p className="no-tables">No tables found in the database</p>
      ) : (
        <div className="tables-list">
          {data?.tables.map((table: TableInfo) => (
            <div key={table.name} className="table-item">
              <div 
                className="table-header"
                onClick={() => toggleTable(table.name)}
              >
                <span className="toggle-icon">
                  {expandedTables.has(table.name) ? '▼' : '▶'}
                </span>
                <span className="table-name">{table.name}</span>
                <span className="column-count">
                  ({table.columns.length} columns)
                </span>
              </div>
              {expandedTables.has(table.name) && (
                <div className="columns-list">
                  {table.columns.map(column => (
                    <div key={column.name} className="column-item">
                      <span className="column-name">{column.name}</span>
                      <span className="column-type">{column.data_type}</span>
                      {column.is_nullable && (
                        <span className="nullable-badge">nullable</span>
                      )}
                    </div>
                  ))}
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
};