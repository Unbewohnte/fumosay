/*
Copyright © 2021, 2022 Kasyanov Nikolay Alexeyevich (Unbewohntes)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

#include <string.h>
#include <iostream>
#include <fstream>
#include <filesystem>

class Fumo {
    private:
    const std::string m_message_identificator = "!message";
    const std::string m_default_fumofile = "cirno.fumo";
    
    std::string m_fumotext;

    public:
    // Opens specified fumofile
    Fumo(std::string fumo_path) {
        std::fstream m_fumofile;
        m_fumofile.open(fumo_path, std::ios_base::in);
        if (!m_fumofile) {
            throw "Could not open fumofile";
        };

        // read whole fumofile
        std::string line;
        while (std::getline(m_fumofile, line)) {
            m_fumotext += line + "\n";
        };

        m_fumofile.close();
    };

    ~Fumo() {};

    // Inserts given message and prints it to the standart output 
    void say(std::string message) {
        if (m_fumotext.find(m_message_identificator) == std::string::npos) {
            // no identificator found. Inserting message on the first line
            m_fumotext.insert(0, message + "\n");
        } else {
            // identificator found. Inserting message in the appropriate place
            const unsigned int message_index = m_fumotext.find(m_message_identificator);
            m_fumotext.replace(message_index, m_message_identificator.length(), message);
        };

        // print fumosay
        std::cout << m_fumotext << std::endl;
    };
};

int main(int argc, char *argv[]) {
    const std::string VERSION = "0.4.5";

    const std::string ABOUT = "\
fumosay\n\
cowsay, but with soft friends\n\n\
(c) Kasyanov Nikolay Alexeyevich (Unbewohnte)";
    
    const std::string HELP = "\
fumosay [OPTION(s)...] [MESSAGE]\n\n\
Options:\n\
-v --version - print version text and exit\n\
-l --list - list available fumos and exit\n\
-h --help - print this message and exit\n\
-f --fumo - specify path to another fumo to print\n";

    std::string fumofiles_dir;
    #ifdef _WIN32
        fumofiles_dir = ".\\fumofiles";
    #else
        fumofiles_dir = "/usr/local/share/fumosay/fumofiles"; 
    #endif
    std::string fumoname = "cirno.fumo";

    if (argc < 2) {
        return 0;
    };

    std::string message;
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "-v") == 0 || strcmp(argv[i], "--version") == 0) {
            // print version info
            std::cout << VERSION << std::endl << std::endl << ABOUT << std::endl;
            return 0;
        }
        else if (strcmp(argv[i], "-l") == 0 || strcmp(argv[i], "--list") == 0) {
            // print available fumos in the default fumofiles directory
            for (const auto &entry : std::filesystem::directory_iterator(fumofiles_dir)) {
                std::cout << entry.path().filename() << std::endl;
            };
            return 0;
        } 
        else if (strcmp(argv[i], "-h") == 0 || strcmp(argv[i], "--help") == 0) {
            // print help infos
            std::cout << HELP << std::endl;
            return 0;
        }
        else if (strcmp(argv[i], "-f") == 0 || strcmp(argv[i], "--fumo") == 0) {
            // use another fumo
            if (i+1 >= argc-1) {
                return 1;
            };

            fumoname = argv[i+1];
            i++;
        }
        else if (strcmp(argv[i], "-d") == 0 || strcmp(argv[i], "--directory") == 0) {
            // temporarily change fumofiles directory to look fumofiles in
            if (i+1 >= argc-1 || !std::filesystem::exists(std::filesystem::path(argv[i+1]))){
                return 1;
            };

            fumofiles_dir = argv[i+1];
            i++;
        }
        else {
            message += argv[i];
            if (i != argc-1) {
                message += " ";
            };
        }
    };

    Fumo fumo(std::filesystem::path(fumofiles_dir) / fumoname);
    fumo.say(message);

    return 0;
}