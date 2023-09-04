package main

import (
	"encoding/json"
	"net/http"
	"os"
)

func closeHandler(resp http.ResponseWriter) {
	if err := recover(); err != nil {
		resp.WriteHeader(http.StatusInternalServerError)
	}
}

func errorHandler(err error) {
	if err != nil {
		panic(err)
	}
}

func syncHandler(resp http.ResponseWriter, req *http.Request) {
	defer closeHandler(resp)
	files, err := os.ReadDir("uploads")
	errorHandler(err)
	output := map[string][]string{"files": {}}
	for _, file := range files {
		output["files"] = append(output["files"], file.Name())
	}
	jsonResp, err := json.Marshal(output)
	errorHandler(err)
	resp.Write(jsonResp)
}

func uploadHandler(resp http.ResponseWriter, req *http.Request) {
	defer closeHandler(resp)
	errorHandler(req.ParseMultipartForm(10 << 20))
	part := req.MultipartForm.File["upload"][0]
	file, err := part.Open()
	errorHandler(err)
	contents := make([]byte, part.Size)
	file.Read(contents)
	errorHandler(os.WriteFile("uploads/"+part.Filename, contents, 0644))
	syncHandler(resp, req)
}

func deleteHandler(resp http.ResponseWriter, req *http.Request) {
	defer closeHandler(resp)
	errorHandler(os.Remove("uploads/" + req.URL.Query().Get("filename")))
	syncHandler(resp, req)
}

func retrieveHandler(resp http.ResponseWriter, req *http.Request) {
	defer closeHandler(resp)
	contents, err := os.ReadFile("uploads/" + req.URL.Query().Get("filename"))
	errorHandler(err)
	resp.Write(contents)
}

func main() {
	http.Handle("/", http.FileServer(http.Dir("dist")))
	http.HandleFunc("/sync", syncHandler)
	http.HandleFunc("/upload-file", uploadHandler)
	http.HandleFunc("/delete", deleteHandler)
	http.HandleFunc("/retrieve", retrieveHandler)
	http.ListenAndServe(":9002", nil)
}
