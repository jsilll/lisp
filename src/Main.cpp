#include <iostream>

#include "Parser.hpp"

int main()
{
    lisp::Environment env;

    while (true)
    {
        std::cout << "> " << std::flush;

        std::string source;
        std::getline(std::cin, source);
        lisp::Lexer lexer(std::move(source));
        lisp::Parser parser(std::move(lexer));

        try
        {
            auto value = parser.Parse();
            std::cout << value.ToString() << '\n';
            const auto result = value.Eval(env);
        }
        catch (const std::exception &e)
        {
            std::cerr << e.what() << '\n';
        }
    }

    return EXIT_SUCCESS;
}