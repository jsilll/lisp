#include "Parser.hpp"

namespace lisp
{
    Value Parser::Parse()
    {
        while (true)
        {
            const auto token = m_lexer.Next();
            switch (token.type)
            {
            case Token::Type::LeftParen:
                return ParseList();
            case Token::Type::RightParen:
                throw std::runtime_error("Unexpected ')'");
            case Token::Type::Quote:
                return Parse().Quote();
            case Token::Type::Symbol:
                return Value(std::string(token.lex), Type::Atom);
            case Token::Type::Number:
                return ParseNumber(token);
            case Token::Type::String:
                return Value(std::string(token.lex), Type::String);
            case Token::Type::Invalid:
                throw std::runtime_error("Invalid token");
            case Token::Type::Eof:
                return {};
            }
        }
        return {};
    }

    Value Parser::ParseList()
    {
        std::vector<Value> values;
        while (true)
        {
            const auto token = m_lexer.Next();
            switch (token.type)
            {
            case Token::Type::RightParen:
                return Value(std::move(values));
            case Token::Type::Invalid:
                throw std::runtime_error("Invalid token");
            case Token::Type::Eof:
                throw std::runtime_error("Unexpected end of input");
            default:
                while (m_lexer.Peek().type != Token::Type::RightParen and m_lexer.Peek().type != Token::Type::Eof)
                {
                    values.emplace_back(Parse());
                }
            }
        }
    }

    Value Parser::ParseNumber(const Token &token)
    {
        if (token.type != Token::Type::Number)
        {
            throw std::runtime_error("Expected number");
        }
        else
        {
            try
            {
                return Value(std::stoi(std::string(token.lex)));
            }
            catch (const std::exception &)
            {
                try
                {
                    return Value(std::stof(std::string(token.lex)));
                }
                catch (const std::exception &)
                {
                    throw std::runtime_error("Invalid number");
                }
            }
        }
    }

} // namespace lisp
