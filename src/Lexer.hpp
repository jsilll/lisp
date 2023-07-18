#pragma once

#include <optional>

#include "Token.hpp"

namespace lisp
{
    /// @brief A lexer for the Lisp language.
    class Lexer final
    {
    public:
        /// @brief Constructs a new lexer.
        Lexer(std::string source) noexcept : m_source(std::move(source)) {}

        /// @brief Gets the next token.
        Token Next() noexcept;

        /// @brief Peeks at the next token.
        Token Peek() noexcept;

    private:
        /// @brief Returns whether the given character is a punctuation character.
        static bool IsPunctuation(const char c) noexcept;

        /// @brief Advances a character.
        void AdvanceChar() noexcept;

        /// @brief Skips whitespace.
        void SkipWhitespace() noexcept;

        /// @brief  Skips a line.
        void SkipLine() noexcept;

        /// @brief Scans a single character.
        Token ScanSingleChar(Token::Type type, std::string_view lex) noexcept;

        /// @brief Scans a number.
        Token ScanNumber() noexcept;

        /// @brief Scans a symbol.
        Token ScanSymbol() noexcept;

        /// @brief Scans a string.
        Token ScanString() noexcept;

        Position m_position;
        std::string m_source;
        std::size_t m_index{0};
        std::optional<Token> m_peek;
    };
} // namespace lisp
