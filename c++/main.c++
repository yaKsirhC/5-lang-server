#include <string>
#include "crow_all.h"
#include <filesystem>
#include <vector>
#include <regex>
#include "json.hpp"
#include "crow-fileupload.h"

using namespace crow;
using namespace std;
namespace fs = std::filesystem;
using nljson = nlohmann::json;

#define uploadDir "./uploads/"

nljson readUploadDir(){
    nljson jsonObj;
    nljson jsonArr;
    for(const auto& file: fs::directory_iterator(uploadDir)){
        jsonArr.push_back(file.path().filename().c_str());
    };
    jsonObj["files"] = jsonArr;
    return jsonObj;
};

int main(){
    crow::SimpleApp app;
    CROW_ROUTE(app, "/")([](const request& req, response& res){
        res.set_static_file_info("./dist/index.html");

        return res.end();
    });
    CROW_ROUTE(app, "/sync")([](const request& req, response& res){
        try
        {
            nljson files = readUploadDir();
            res.set_header("Content-Type", "application/json");
            res.write(files.dump());
            response(200);
            res.end();
        }
        catch(const std::exception& e)
        {
            std::cerr << e.what() << '\n';
            response(500);
        }
        
    });
    app.route_dynamic("/upload-file").methods("POST"_method)([](const request& req, response& res){
        try
        {
            const string contType = req.get_header_value("Content-Type");
            if(!(contType.rfind("multipart/form-data",0) == 0)) {
                response(400);
                res.end();
                return;
            }
            fileupload::fileupload filesReq(req);
            filesReq.saveAllSelected("upload");

            nljson files = readUploadDir();
            res.set_header("Content-Type", "application/json");
            res.write(files.dump());
            response(200);
            res.end();
        }
        catch(const std::exception& e)
        {
            std::cerr << e.what() << '\n';
            response(500);res.end();
        }
    
    });
    app.route_dynamic("/delete").methods("DELETE"_method)([](const request& req, response& res){
        try
        {
            const string filename = req.url_params.get("filename");
            // cout<<filename<<endl;
            bool succ = fs::remove(uploadDir+filename);
            nljson files = readUploadDir();
            // cout<<succ;
            if(!succ) {
                response(500);
                res.set_header("Content-Type", "application/json");
                res.write(files.dump());

                return res.end();
            };

            res.set_header("Content-Type", "application/json");
            res.write(files.dump());

            response(200);
            res.end();
        }
        catch(const std::exception& e)
        {
            std::cerr << e.what() << '\n';
            response(400);
            res.end();
        }
        
    });
    app.route_dynamic("/retrieve").methods("GET"_method)([](const request &req, response &res){
        try
        {
            const string filename = req.url_params.get("filename");

            res.set_static_file_info(uploadDir+filename);
            res.end();
        }
        catch(const std::exception& e)
        {
            std::cerr << e.what() << '\n';
            response(500);
            res.end();
        }
        

    });
    app.port(9000).multithreaded().run();

    return 0;
}