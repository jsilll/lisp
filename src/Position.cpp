#include "Position.hpp"

namespace lisp
{
    void Position::Advance(const char c) noexcept
    {
        if (c == '\n')
        {
            ++line;
            column = 1;
        }
        else
        {
            ++column;
        }
    }

    void Position::AdvanceNewline() noexcept
    {
        ++line;
        column = 1;
    }

    std::string Position::ToString() const noexcept
    {
        return std::to_string(line) + ":" + std::to_string(column);
    }

    std::string Position::ToString(const std::string &filename) const noexcept
    {
        return filename + ":" + ToString();
    }
} // namespace lisp
