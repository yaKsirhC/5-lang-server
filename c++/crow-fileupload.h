#include <string>
#include "crow_all.h"
#include <filesystem>
#include <vector>
#include <regex>
#include "json.hpp"
#define DEFAULT_DIR "./uploads/"

using namespace crow;
using namespace std;

namespace fileupload{
    class file : multipart::part{
        public:
            string filename;
            file(const string fileName, multipart::part* part): multipart::part(*part), filename(fileName){}

            int mv() const {
                ofstream file(DEFAULT_DIR+filename, std::ofstream::binary);
                file.write(part::body.data(), part::body.length());
                file.close();
                return 0;
            }
            
    };
    class fileupload: request{
        public:
            map<const string, vector<file>> files;
            fileupload(const request req):request(req){
                string reqBody = this->body;
                multipart::message parts(req);

                std::regex pattern("name=\"([^\"]+)\" *. filename=\"([^\"]+)\"");
                smatch matches;
                int i = 0;
                while (regex_search(reqBody, matches, pattern)) {
                    file newFile(matches[2].str(), &parts.parts[i]);
                    files[matches[1].str()].push_back(newFile);

                    reqBody = matches.suffix().str();
                    i++;
                }
            };
            int saveAllSelected(const char* inputName){
                for(const auto& file: files[inputName]){
                    file.mv();
                }
                return 0;
            };
            int saveAll(){
                for(const auto& specificFiles:files){
                    saveAllSelected(specificFiles.first.c_str());
                }
                return 0;
            };
    };

}