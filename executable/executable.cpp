#include <iostream>

int main(int argc, char **argv)
{
    std::cout << "Begining Executable\n";
    for (auto i = 0; i < argc; i++)
    {
        std::cout << "Arg [" << i << "]: " << argv[i] << '\n';
    }
}