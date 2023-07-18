#pragma once

#include <optional>

#include "Token.hpp"

namespace lisp
{
    class Lexer final
    {
    public:
        Lexer(std::string source) noexcept : m_source(std::move(source)) {}

        Token Next() noexcept;

        Token Peek() noexcept;

    private:
        static bool IsPunctuation(const char c) noexcept;

        void AdvanceChar() noexcept;

        void SkipWhitespace() noexcept;

        void SkipLine() noexcept;

        Token ScanSingleChar(Token::Type type, std::string_view lex) noexcept;

        Token ScanNumber() noexcept;

        Token ScanSymbol() noexcept;

        Token ScanString() noexcept;

        Position m_position;
        std::string m_source;
        std::size_t m_index{0};
        std::optional<Token> m_peek;
    };
} // namespace lisp
