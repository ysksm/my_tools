import axios from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api';

export const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

export interface TableInfo {
  name: string;
  columns: ColumnInfo[];
}

export interface ColumnInfo {
  name: string;
  data_type: string;
  is_nullable: boolean;
}

export interface SchemaResponse {
  tables: TableInfo[];
}

export interface QueryRequest {
  sql: string;
}

export interface QueryResponse {
  columns: string[];
  rows: any[][];
  row_count: number;
}

export const api = {
  getSchema: () => apiClient.get<SchemaResponse>('/schema'),
  
  getTableData: (table: string, limit?: number, offset?: number) => {
    const params = new URLSearchParams({ table });
    if (limit) params.append('limit', limit.toString());
    if (offset) params.append('offset', offset.toString());
    
    return apiClient.get(`/data?${params}`, {
      responseType: 'blob',
    });
  },
  
  executeQuery: (sql: string) => 
    apiClient.post<QueryResponse>('/query', { sql }),
  
  healthCheck: () => apiClient.get('/health'),
};