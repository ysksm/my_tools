package main

import (
	"encoding/base64"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
)

type Config struct {
	JiraBaseUrl  string `json:"jira_base_url"`
	JiraUsername string `json:"jira_username"`
	JiraApiToken string `json:"jira_api_token"`
}

type Project struct {
	ID             string `json:"id"`
	Key            string `json:"key"`
	Name           string `json:"name"`
	ProjectTypeKey string `json:"projectTypeKey"`
}

type ProjectResponse struct {
	Values     []Project `json:"values"`
	StartAt    int       `json:"startAt"`
	MaxResults int       `json:"maxResults"`
	Total      int       `json:"total"`
}

func getBasicAuthHeader(username, apiToken string) string {
	auth := username + ":" + apiToken
	return "Basic " + base64.StdEncoding.EncodeToString([]byte(auth))
}

func getProjects(config Config) (*ProjectResponse, error) {
	client := &http.Client{}
	url := config.JiraBaseUrl + "/rest/api/3/project/search"

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return nil, fmt.Errorf("error creating request: %v", err)
	}

	req.Header.Add("Authorization", getBasicAuthHeader(config.JiraUsername, config.JiraApiToken))
	req.Header.Add("Content-Type", "application/json")

	resp, err := client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("error making request: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return nil, fmt.Errorf("error response from JIRA: status=%d, body=%s", resp.StatusCode, string(body))
	}

	var projectResp ProjectResponse
	if err := json.NewDecoder(resp.Body).Decode(&projectResp); err != nil {
		return nil, fmt.Errorf("error decoding response: %v", err)
	}

	return &projectResp, nil
}

func handleProjects(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	file, err := os.Open("config.json")
	if err != nil {
		http.Error(w, "Error opening config file: "+err.Error(), http.StatusInternalServerError)
		return
	}
	defer file.Close()

	var config Config
	if err := json.NewDecoder(file).Decode(&config); err != nil {
		http.Error(w, "Error decoding config file: "+err.Error(), http.StatusInternalServerError)
		return
	}

	projects, err := getProjects(config)
	if err != nil {
		http.Error(w, "Error getting projects: "+err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(projects)
}

func main() {
	http.HandleFunc("/api/projects", handleProjects)

	port := "8080"
	fmt.Printf("Server starting on port %s...\n", port)
	if err := http.ListenAndServe(":"+port, nil); err != nil {
		log.Fatal(err)
	}
}
