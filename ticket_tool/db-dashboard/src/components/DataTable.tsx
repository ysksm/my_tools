import React, { useState, useEffect } from 'react';
import { QueryResponse } from '../api/client';
import './DataTable.css';

interface DataTableProps {
  data: QueryResponse | null;
}

export const DataTable: React.FC<DataTableProps> = ({ data }) => {
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(50);
  
  useEffect(() => {
    setPage(0);
  }, [data]);

  if (!data) {
    return (
      <div className="data-table-container empty">
        <p>Execute a query to see results</p>
      </div>
    );
  }

  if (data.rows.length === 0) {
    return (
      <div className="data-table-container empty">
        <p>No results found</p>
      </div>
    );
  }

  const totalPages = Math.ceil(data.row_count / pageSize);
  const startRow = page * pageSize;
  const endRow = Math.min(startRow + pageSize, data.row_count);
  const displayRows = data.rows.slice(startRow, endRow);

  return (
    <div className="data-table-container">
      <div className="table-header">
        <div className="table-info">
          Showing {startRow + 1}-{endRow} of {data.row_count} rows
        </div>
        <div className="table-controls">
          <label>
            Rows per page:
            <select 
              value={pageSize} 
              onChange={(e) => {
                setPageSize(Number(e.target.value));
                setPage(0);
              }}
            >
              <option value={10}>10</option>
              <option value={25}>25</option>
              <option value={50}>50</option>
              <option value={100}>100</option>
            </select>
          </label>
        </div>
      </div>
      
      <div className="table-wrapper">
        <table className="data-table">
          <thead>
            <tr>
              {data.columns.map((col, idx) => (
                <th key={idx}>{col}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {displayRows.map((row, rowIdx) => (
              <tr key={rowIdx}>
                {row.map((cell, cellIdx) => (
                  <td key={cellIdx}>
                    {cell === null ? (
                      <span className="null-value">NULL</span>
                    ) : (
                      String(cell)
                    )}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      
      {totalPages > 1 && (
        <div className="pagination">
          <button 
            onClick={() => setPage(0)}
            disabled={page === 0}
          >
            First
          </button>
          <button 
            onClick={() => setPage(page - 1)}
            disabled={page === 0}
          >
            Previous
          </button>
          <span className="page-info">
            Page {page + 1} of {totalPages}
          </span>
          <button 
            onClick={() => setPage(page + 1)}
            disabled={page >= totalPages - 1}
          >
            Next
          </button>
          <button 
            onClick={() => setPage(totalPages - 1)}
            disabled={page >= totalPages - 1}
          >
            Last
          </button>
        </div>
      )}
    </div>
  );
};