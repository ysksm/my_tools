package main

import (
	"encoding/json"
	"fmt"
	"os"
)

type Config struct {
	JiraBaseUrl  string `json:"jira_base_url"`
	JiraUsername string `json:"jira_username"`
	JiraApiToken string `json:"jira_api_token"`
}

func main() {
	file, err := os.Open("config.json")
	if err != nil {
		fmt.Println("Error opening config file:", err)
		return
	}
	defer file.Close()

	var config Config
	if err := json.NewDecoder(file).Decode(&config); err != nil {
		fmt.Println("Error decoding config file:", err)
		return
	}

	fmt.Printf("Jira Base URL: %s\n", config.JiraBaseUrl)
	fmt.Printf("Jira Username: %s\n", config.JiraUsername)
	fmt.Printf("Jira API Token: %s\n", config.JiraApiToken)

}
