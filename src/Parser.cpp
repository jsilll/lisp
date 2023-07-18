#include "Parser.hpp"

#include <stdexcept>
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
            case Token::Type::Integer:
                return ParseInteger(token);
            case Token::Type::Float:
                return ParseFloat(token);
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
            switch (m_lexer.Peek().type)
            {
            case Token::Type::RightParen:
                m_lexer.Next();
                return Value(std::move(values));
            case Token::Type::Invalid:
                throw std::runtime_error("Invalid token");
            case Token::Type::Eof:
                throw std::runtime_error("Unexpected end of input");
            default:
                do
                {
                    values.emplace_back(Parse());
                } while (m_lexer.Peek().type != Token::Type::RightParen and m_lexer.Peek().type != Token::Type::Eof);
                if (m_lexer.Peek().type == Token::Type::Eof)
                {
                    throw std::runtime_error("Unexpected end of input");
                }
                else
                {
                    m_lexer.Next();
                    return Value(std::move(values));
                }
            }
        }
    }

    Value Parser::ParseInteger(const Token &token)
    {
        if (token.type != Token::Type::Integer)
        {
            throw std::runtime_error("Expected an integer");
        }
        else
        {
            try
            {
                return Value(std::stoi(std::string(token.lex)));
            }
            catch (const std::exception &)
            {
                throw std::runtime_error("Invalid integer literal");
            }
        }
    }

    Value Parser::ParseFloat(const Token &token)
    {
        if (token.type != Token::Type::Float)
        {
            throw std::runtime_error("Expected a float");
        }
        try
        {
            return Value(std::stof(std::string(token.lex)));
        }
        catch (const std::exception &)
        {
            throw std::runtime_error("Invalid float literal");
        }
    }

} // namespace lisp
