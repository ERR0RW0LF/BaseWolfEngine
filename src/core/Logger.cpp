#include "Logger.h"
#include <iostream>

void Logger::Log(const std::string& message) {
    std::cout << message << std::endl;
}