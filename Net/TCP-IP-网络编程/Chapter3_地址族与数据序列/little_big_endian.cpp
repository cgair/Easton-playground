#include <iostream>

union EndianChecker
{   
    int val;
    char bytes[sizeof(int)];
};


int main()
{
    // 1
    int value = 0x12345678;
    
    char* ptr = reinterpret_cast<char*>(&value);
    
    if (*ptr == 0x12) std::cout << "Big-Endian.\n";
    else if (*ptr == 0x78) std::cout << "Little-Endian.\n";
    
    // 2
    EndianChecker ec;
    ec.val = value;
    std::cout << "Value: " << std::hex << value << std::endl;

    if (ec.bytes[0] == 0x12) {
        std::cout << "Big-endian" << std::endl;
    } else {
        std::cout << "Little-endian" << std::endl;
    }


    return 0;
}