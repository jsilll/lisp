#include <iostream>

#include <lisp/Parser.hpp>
#include <lisp/Standard.hpp>

int main()
{
    lisp::Environment env;
    lisp::std::Register(env);

    while (true)
    {
        std::cout << ">>> " << std::flush;

        std::string source;
        std::getline(std::cin, source);
        lisp::Lexer lexer(std::move(source));
        lisp::Parser parser(std::move(lexer));

        try
        {
            auto value = parser.Parse();
            const auto result = value.Eval(env);
            std::cout << result.ToString() << '\n';
        }
        catch (const std::exception &e)
        {
            std::cerr << e.what() << '\n';
        }
    }

    return EXIT_SUCCESS;
}