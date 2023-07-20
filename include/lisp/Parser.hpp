#pragma once

#include <lisp/Lexer.hpp>
#include <lisp/Value.hpp>

namespace lisp
{
    /// @brief A parser for the Lisp language.
    class Parser final
    {
    public:
        /// @brief Constructs a parser from a source string.
        explicit Parser(Lexer lexer) noexcept : m_lexer(::std::move(lexer)) {}

        /// @brief Parses a source string.
        Value Parse();

    private:
        /// @brief Parses a value.
        Value ParseValue();

        /// @brief Parses a list of values.
        Value ParseList();

        /// @brief Parses an integer number.
        Value ParseInteger(const Token &token);

        /// @brief Parses a float number.
        Value ParseFloat(const Token &token);

        Lexer m_lexer;
    };
} // namespace lisp
