#include <lisp/Lexer.hpp>

namespace lisp
{
    Token Lexer::Next() noexcept
    {
        if (m_peek.has_value())
        {
            const auto token = m_peek.value();
            m_peek.reset();
            return token;
        }

        SkipWhitespace();
        if (m_index >= m_source.size())
        {
            return Token{Token::Type::Eof, m_position, ""};
        }
        else
        {
            while (m_index < m_source.size() and m_source[m_index] == ';')
            {
                SkipLine();
                SkipWhitespace();
            }

            if (m_index >= m_source.size())
            {
                return Token{Token::Type::Eof, m_position, ""};
            }
            else
            {
                switch (m_source[m_index])
                {
                case '\'':
                    return ScanSingleChar(Token::Type::Quote, "'");
                case '(':
                    return ScanSingleChar(Token::Type::LeftParen, "(");
                case ')':
                    return ScanSingleChar(Token::Type::RightParen, ")");
                case '"':
                    return ScanString();
                default:
                    if (std::isdigit(m_source[m_index]))
                    {
                        return ScanNumber();
                    }
                    else
                    {
                        return ScanSymbol();
                    }
                }
            }
        }
    }

    Token Lexer::Peek() noexcept
    {
        if (!m_peek.has_value())
        {
            m_peek = Next();
        }
        return m_peek.value();
    }

    bool Lexer::IsPunctuation(const char c) noexcept
    {
        return c == '(' or c == ')' or c == '\'' or c == '"' or c == ';';
    }

    void Lexer::AdvanceChar() noexcept
    {
        ++m_index;
        ++m_position.column;
    }

    void Lexer::AdvanceNewline() noexcept
    {
        ++m_index;
        m_position.AdvanceNewline();
    }

    void Lexer::SkipWhitespace() noexcept
    {
        while (m_index < m_source.size() and std::isspace(m_source[m_index]))
        {
            m_position.Advance(m_source[m_index++]);
        }
    }

    void Lexer::SkipLine() noexcept
    {
        while (m_index < m_source.size() and m_source[m_index] != '\n')
        {
            AdvanceChar();
        }
    }

    Token Lexer::ScanSingleChar(const Token::Type type, const std::string_view& lex) noexcept
    {
        const auto pos = m_position;
        AdvanceChar();
        return Token{type, pos, lex};
    }

    Token Lexer::ScanNumber() noexcept
    {
        const auto start = m_index;
        const auto pos = m_position;
        while (m_index < m_source.size() and std::isdigit(m_source[m_index]))
        {
            AdvanceChar();
        }
        if (m_index < m_source.size() and m_source[m_index] == '.')
        {
            AdvanceChar();
            while (m_index < m_source.size() and std::isdigit(m_source[m_index]))
            {
                AdvanceChar();
            }
            const auto lex = std::string_view(&m_source[start], m_index - start);
            return Token{Token::Type::Float, pos, lex};
        }
        else
        {
            const auto lex = std::string_view(&m_source[start], m_index - start);
            return Token{Token::Type::Integer, pos, lex};
        }
    }

    Token Lexer::ScanSymbol() noexcept
    {
        const auto start = m_index;
        const auto pos = m_position;
        while (m_index < m_source.size() and !IsPunctuation(m_source[m_index]) and !std::isspace(m_source[m_index]))
        {
            AdvanceChar();
        }
        const auto lex = std::string_view(&m_source[start], m_index - start);
        return lex.size() == 0 ? Token{Token::Type::Invalid, pos, lex} : Token{Token::Type::Symbol, pos, lex};
    }

    Token Lexer::ScanString() noexcept
    {
        AdvanceChar();
        const auto start = m_index;
        const auto pos = m_position;
        while (m_index < m_source.size() and m_source[m_index] != '"')
        {
            if (m_source[m_index] == '\n')
            {
                AdvanceNewline();
                return Token{Token::Type::Invalid, pos, "\n"};
            }
            else
            {
                AdvanceChar();
            }
        }
        if (m_source[m_index] != '"')
        {
            return Token{Token::Type::Invalid, pos, ""};
        }
        else
        {
            const auto lex = std::string_view(&m_source[start], m_index - start);
            AdvanceChar();
            return Token{Token::Type::String, pos, lex};
        }
    }
} // namespace lisp
