#pragma once

#include <string>

namespace lisp
{
    struct Position
    {
        /// @brief Advances the position by one character.
        void Advance(const char c) noexcept;

        /// @brief Advances the position by a newline.
        void AdvanceNewline() noexcept;

        /// @brief Converts the position to a string.
        std::string ToString() const noexcept;

        /// @brief Converts the position to a string with a filename.
        std::string ToString(const std::string &filename) const noexcept;

        std::size_t line{1};
        std::size_t column{1};
    };
} // namespace lisp
