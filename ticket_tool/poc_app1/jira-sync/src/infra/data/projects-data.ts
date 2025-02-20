import { firstValueFrom } from "rxjs";
import { ProjectSyncData, type ConnectSetting, type ProjectInfo } from "../connect-setting";
import * as fs from 'fs';
import * as Path from 'path';
import { DATA_DIR, FIELDS_FILE_PATH, USER_FILE_PATH, USER_FILE_PATH2, USER_FILE_PATH3, USER_FILE_PATH4, USER_FILE_PATH5 } from "../const-definitions";
import { IssueSearch, type Fields, type Issue, type SearchRequest } from "../api/issue-search";
import * as duckdb from "duckdb";
import { SchemaData, type FieldData } from "./issue-field-data";
import { Logger } from "../log/logger";

const logger = Logger.getInstance();


export const ProjectsData = {

    // データベースの初期化
    Initialize: async (connectionSetting: ConnectSetting) => {
        // isSyncがtrueのプロジェクトのみ処理を行う
        const syncProjects = connectionSetting.projectInfos.filter(p => p.isSync);
        const fields = ['*all'];
        const startAt = 0;
        const maxResults = 100;
        let total = 0;
        for (const project of syncProjects) {
            const project_dir = Path.join(DATA_DIR, project.projectKey);
            if (!fs.existsSync(project_dir)) {
                fs.mkdirSync(project_dir, { recursive: true });
            }

            let fieldsDefinitons: string[] = [];
            let ViewFields: string[] = [];
            ViewFields.push('id', 'key', 'expand', 'self');
            const fieldsValue = await SchemaData.ReadData();
            
            fieldsValue.forEach(field => {
                switch (field.schema?.type) {
                    case 'number':
                        fieldsDefinitons.push(`${field.id} INTEGER NULL`);
                        ViewFields.push(`${field.id} as '${field.name}'`);
                        break;                        
                    case 'string':
                    case 'date':
                    case 'datetime':
                        fieldsDefinitons.push(`${field.id} ${field.schema.type} NULL`);
                        ViewFields.push(`${field.id} as '${field.name}'`);
                        break;
                    case 'project':
                    case 'issuetype':
                    case 'priority':
                    case 'status':
                        fieldsDefinitons.push(`${field.id}_id INTEGER NULL`);
                        ViewFields.push(`${field.id}_id as '${field.name}_id'`);
                        break;
                    case 'user':
                        fieldsDefinitons.push(`${field.id}_accountId string NULL`);
                        ViewFields.push(`${field.id}_accountId as '${field.name}_accountId'`);
                        break;
                    case 'array':
                    case 'any':
                        fieldsDefinitons.push(`${field.id} JSON`);
                        ViewFields.push(`${field.id} as '${field.name}'`);
                        break;
                }
                
            });
            const duckDbFilePath = Path.join(project_dir, 'data.duckdb');
            const db = new duckdb.Database(duckDbFilePath);
            const connect = db.connect();
            
            await createStatusCategory(connect);
            await createIssueTypes(connect);
            await createProjects(connect);
            await createPriorities(connect);
            await createUsers(connect);
            
            try{
                const fieldsDefinitonsStr = fieldsDefinitons.join(', ');
                const crewateIssuesSql = `CREATE TABLE IF NOT EXISTS issues (id INTEGER PRIMARY KEY, key VARCHAR, expand VARCHAR, self VARCHAR, fields JSON, ${fieldsDefinitonsStr});`;
                await connect.exec(crewateIssuesSql, function(err, res){
                    if(err){
                        logger.error(err.message + ' ' + crewateIssuesSql);
                    }
                });
                const createViewSql = `CREATE VIEW IF NOT EXISTS issues_view AS SELECT ${ViewFields.join(', ')} FROM issues;`;
                await connect.exec(createViewSql, function(err, res){
                    if(err){
                        logger.error(err.message + ' ' + createViewSql);
                    }
                });

            }catch(e){
                console.error(e);
            }finally{
                connect.close();
                db.close();

                // 1秒待機
                await new Promise(resolve => setTimeout(resolve, 1000));
            }
        }
    },
    
    // プロジェクト情報を同期する
    SyncData: async (connectionSetting: ConnectSetting) => {
        const logger = Logger.getInstance();
        // isSyncがtrueのプロジェクトのみ処理を行う
        const syncProjects = connectionSetting.projectInfos.filter(p => p.isSync);
        const fields = ['*all'];

        for (const project of syncProjects) {
            logger.info(`Project: ${project.projectKey} ${project.projectName} Data Sync Start.`);
            const project_dir = Path.join(DATA_DIR, project.projectKey);
            if (!fs.existsSync(project_dir)) {
                fs.mkdirSync(project_dir, { recursive: true });
            }
            await syncProject(project_dir, project, connectionSetting, fields, logger);
        }
    }
}

async function createUsers(connect: duckdb.Connection) {
    const createTableSQL = `CREATE TABLE IF NOT EXISTS users 
                                        (
                                            self VARCHAR NULL, 
                                            accountId VARCHAR PRIMARY KEY, 
                                            accountType VARCHAR NULL,
                                            emailAddress VARCHAR NULL,
                                            avatarUrls VARCHAR NULL,
                                            displayName VARCHAR NULL,
                                            active BOOLEAN NULL,
                                            locale VARCHAR NULL
                                            );`;
    await executeSQL(connect, createTableSQL);

    const insert = async (filePath: string) => {
        const copyTable = `INSERT OR REPLACE INTO users SELECT 
        self , 
        accountId , 
        accountType ,
        emailAddress ,
        avatarUrls ,
        displayName ,
        active ,
        locale 
        FROM read_json('${filePath}');`;
        await executeSQL(connect, copyTable);
    }
    await insert(USER_FILE_PATH);
    if(fs.existsSync(USER_FILE_PATH2)){
        await insert(USER_FILE_PATH2);
    }
    if(fs.existsSync(USER_FILE_PATH3)){
        await insert(USER_FILE_PATH3);
    }
    if(fs.existsSync(USER_FILE_PATH4)){
        await insert(USER_FILE_PATH4);
    }
    if(fs.existsSync(USER_FILE_PATH5)){
        await insert(USER_FILE_PATH5);
    }
}

async function createPriorities(connect: duckdb.Connection) {
    const createTableSQL = `CREATE TABLE IF NOT EXISTS priorities 
                                        (
                                            self VARCHAR, 
                                            statusColor VARCHAR,
                                            description VARCHAR, 
                                            iconUrl VARCHAR,
                                            name VARCHAR,
                                            id LONG PRIMARY KEY, 
                                            );`;
    await executeSQL(connect, createTableSQL);
    const copyTable = `INSERT OR REPLACE INTO priorities SELECT * FROM read_json('${Path.join(DATA_DIR, 'priorities.json')}');`;
    await executeSQL(connect, copyTable);
}

async function createProjects(connect: duckdb.Connection) {
    const createTableSQL = `CREATE TABLE IF NOT EXISTS projects 
                                        (
                                            expand VARCHAR, 
                                            self VARCHAR, 
                                            id LONG PRIMARY KEY, 
                                            key VARCHAR,
                                            name VARCHAR,
                                            avatarUrls JSON,
                                            projectTypeKey VARCHAR,
                                            simplified BOOLEAN,
                                            style VARCHAR,
                                            isPrivate boolean
                                            );`;
    await executeSQL(connect, createTableSQL);
    const copyTable = `INSERT OR REPLACE INTO projects SELECT 
                                            expand , 
                                            self , 
                                            id , 
                                            key ,
                                            name ,
                                            avatarUrls ,
                                            projectTypeKey ,
                                            simplified ,
                                            style ,
                                            isPrivate 
                                            FROM read_json('${Path.join(DATA_DIR, 'projects.json')}');`;
    await executeSQL(connect, copyTable);
}

async function createIssueTypes(connect: duckdb.Connection) {
    const createTableSQL = `CREATE TABLE IF NOT EXISTS issuetypes 
                                        (
                                            self VARCHAR, 
                                            id LONG PRIMARY KEY, 
                                            description VARCHAR, 
                                            iconUrl VARCHAR, 
                                            name VARCHAR,
                                            untranslatedName VARCHAR,
                                            subtask boolean,
                                            avatarId LONG,
                                            hierarchyLevel LONG,
                                            );`;
    await executeSQL(connect, createTableSQL);
    const copyTable = `INSERT OR REPLACE INTO issuetypes SELECT
                                            self , 
                                            id , 
                                            description , 
                                            iconUrl , 
                                            name ,
                                            untranslatedName ,
                                            subtask ,
                                            avatarId ,
                                            hierarchyLevel 
                                            FROM read_json('${Path.join(DATA_DIR, 'issuetypes.json')}');`;
    await executeSQL(connect, copyTable);
}

async function createStatusCategory(connect: duckdb.Connection) {
    const createTableSQL = `CREATE TABLE IF NOT EXISTS statuscategory (self VARCHAR, id LONG PRIMARY KEY, key VARCHAR, colorName VARCHAR, name VARCHAR);`;
    await executeSQL(connect, createTableSQL);
    // const copyTable = `COPY statuscategory FROM '${Path.join(DATA_DIR, 'statuscategory.json')}'  (FORMAT 'json', auto_detect TRUE);`;
    const copyTable = `INSERT OR REPLACE INTO statuscategory SELECT * FROM read_json('${Path.join(DATA_DIR, 'statuscategory.json')}');`;
    await executeSQL(connect, copyTable);
}

async function executeSQL(connect: duckdb.Connection, crewateStatusCategorySql: string) {
    await connect.exec(crewateStatusCategorySql, function (err, res) {
        if (err) {
            logger.error(err.message + ' ' + crewateStatusCategorySql);
        }
    });
}

async function syncProject(project_dir: string, project: ProjectInfo, connectionSetting: ConnectSetting, fields: string[], logger: Logger) {
    const duckDbFilePath = Path.join(project_dir, 'data.duckdb');
    const db = new duckdb.Database(duckDbFilePath);
    const connect = db.connect();

    const fieldsValue = await SchemaData.ReadData();
    // プロジェクトの同期情報を取得
    let projectSyncData = ProjectSyncData.loadProject(project);

    // 最終更新日時を取得
    let total = 0;

    let isStop = false;
    while (!isStop) {
        const jql = createJql(projectSyncData.lastUpdated, projectSyncData, project);
        const requestBody: SearchRequest = createRequestBody(jql, fields);

        const result = await firstValueFrom(IssueSearch.Post(connectionSetting.credentialInfo, requestBody));
        if (result === null) {
            break;
        }

        for (const issue of result.issues) {

            // ファイルへ書き出し
            writeIssuJson(project_dir, issue);
            
            const fieldObjects = JSON.parse(JSON.stringify(issue.fields));
            // データベースへ登録
            if(!(await updateDb(issue, fieldsValue, fieldObjects, connect))){
                logger.error(`Project: ${project.projectKey} ${project.projectName} Data Sync Error. IssueKey: ${issue.key}`);
                isStop = true;
                break;
            }
            
            /// 最終更新日時の更新
            // 更新日時　"updated": "2024-08-27T00:41:24.208+0900"　から　2024-08-26T15:41:24.208Zに変換
            let issueLastUpdated = new Date((fieldObjects as Fields).updated);
            if (projectSyncData.lastUpdated.toISOString().slice(0, 16).replace('T', ' ') !== issueLastUpdated.toISOString().slice(0, 16).replace('T', ' ')) {
                projectSyncData.lastupdatedIssueKeys = [];
            }
            projectSyncData.lastupdatedIssueKeys.push(issue.key);
            projectSyncData.lastUpdated = issueLastUpdated;
        }
        ProjectSyncData.saveProjectSyncData(projectSyncData);
        total += result.issues.length;
        if (result.total === 0 || result.total === result.maxResults) {
            // データ同期完了のメッセージ。取得できた件数を表示する。
            logger.info(`Project: ${project.projectKey} ${project.projectName} Data Sync Completed. Total: ${total}`);
            break;
        }else{
            logger.info(`next`);
        }
    }
    // showAllData(connect);
    try {
        await connect.close();
        await db.close();
    }catch(e){
        console.error(e);
        logger.error('error');
    }
}
async function updateDb(issue: Issue, fieldsValue: FieldData[], fieldObjects: any, connect: duckdb.Connection): Promise<boolean> {
    const insertSQL = createInsertSQL(issue, fieldsValue, fieldObjects);
    return new Promise((resolve, reject) => {
        connect.exec(insertSQL, function (err, res) {
            if (err) {
                const logger = Logger.getInstance();
                logger.info(insertSQL);
                logger.error(err.message);
                resolve(false);
            }else{
                resolve(true);
            }
            
        });
    });
}

function writeIssuJson(project_dir: string, issue: Issue) {
    const issueFilePath = Path.join(project_dir, `${issue.key}.json`);
    fs.writeFileSync(issueFilePath, JSON.stringify(issue), 'utf8');
}

function createRequestBody(jql: string, fields: string[]): SearchRequest {
    const startAt = 0;
    const maxResults = 100;
    return {
        jql: jql,
        startAt: startAt,
        maxResults: maxResults,
        fields: fields,
        expand: ['changelog']
    };
}

function showAllData(connect: duckdb.Connection) {
    connect.all('SELECT id, key, summary, updated FROM issues;', function (err, res) {
        if (err) {
            console.log(err);
        }

        for (const row of res) {
            console.log(row.id, row.key, row.summary, row.updated);
        }
    });
}

function createInsertSQL(issue: Issue, fieldsValue: FieldData[], fieldObjects: any) {
    let fieldNames = ['id', 'key', 'expand', 'self'];
    let fieldValues = [issue.id, `'${issue.key}'`, `'${issue.expand}'`, `'${issue.self}'`];
    for (const field of fieldsValue) {
        let { fieldName, fieldValue }: { fieldName: string; fieldValue: string; } = createFieldNameAndValue(field, fieldObjects);
        if (fieldName !== '') {
            fieldNames.push(fieldName);
            fieldValues.push(fieldValue);
        }
    }

    // DuckDbにデータを登録
    const insertSQL = `INSERT OR REPLACE INTO issues(${fieldNames.join(',')}) VALUES (${fieldValues.join(',')});`;
    return insertSQL;
}

function createFieldNameAndValue(field: FieldData, fieldObjects: any) {
    let fieldName: string = '';
    let fieldValue: string = '';
    switch (field.schema?.type) {
        case 'string':
            fieldName = `${field.id}`;
            if(fieldObjects[field.id] !== null && fieldObjects[field.id] !== undefined){
                if(typeof fieldObjects[field.id] === 'string'){
                    fieldValue = `E'${fieldObjects[field.id].replace(/'/g, "¥'")}'`;
                }else{
                    fieldValue = 'null';
                }
            }else if(fieldObjects[field.id] === null){
                fieldValue = 'null';
            }else if(typeof fieldObjects[field.id] !== 'string'){
                fieldValue = 'null';
            }else{
                fieldValue = `'${fieldObjects[field.id].replace("'", "''")}'`;
            }
            break;
        case 'number':
        case 'date':
        case 'datetime':
            fieldName = `${field.id}`;
            fieldValue = fieldObjects[field.id] === null || fieldObjects[field.id] === undefined ? 'null' : `'${fieldObjects[field.id]}'`;
            break;
        case 'project':
        case 'issuetype':
        case 'priority':
        case 'status':
            fieldName = `${field.id}_id`;
            fieldValue = fieldObjects[field.id].id;
            break;
        case 'user':
            if(fieldObjects[field.id] !== null){
                fieldName = `${field.id}_accountId`;
                fieldValue = `'${fieldObjects[field.id].accountId}'`;
            }
            break;
        case 'array':
        case 'any':
            if(fieldObjects[field.id] !== null && fieldObjects[field.id] !== undefined){
                fieldName = `${field.id}`;
                fieldValue = `'${JSON.stringify(fieldObjects[field.id]).replace("'", "''")}'`;
            }
            break;
    }
    return { fieldName, fieldValue };
}

function createJql(lastUpdated: Date, projectSyncData: ProjectSyncData, project: ProjectInfo) {
    let lastUpdatedJST = new Date(lastUpdated.toISOString());
    // JSTの日時の文字列を取得するために9時間足す
    lastUpdatedJST.setHours(lastUpdatedJST.getHours() + 9);
    const lastUpdatedStrJST = lastUpdatedJST.toISOString().slice(0, 16).replace('T', ' ');

    // 除外するIssueKeyを取得
    let excludeIssueKeys = '';
    if (projectSyncData.lastupdatedIssueKeys.length > 0) {
        excludeIssueKeys = ` AND key NOT IN ('${projectSyncData.lastupdatedIssueKeys.join("','")}')`;
    }

    // JQLを作成
    const jql = `${project.whereCondition} AND updated >= '${lastUpdatedStrJST}'${excludeIssueKeys} ORDER BY ${project.orderBy}`;
    return jql;
}

