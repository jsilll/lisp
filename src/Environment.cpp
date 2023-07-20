#include <lisp/Environment.hpp>

#include <lisp/Value.hpp>

namespace lisp
{
    std::optional<Value> Environment::Get(const std::string &name) const noexcept
    {
        if (const auto it = m_definitions.find(name); it != m_definitions.end())
        {
            return it->second;
        }
        else if (m_parent)
        {
            return m_parent->Get(name);
        }
        else
        {
            return std::nullopt;
        }
    }

    void Environment::Combine(const Environment &other) noexcept
    {
        for (const auto &[name, value] : other.m_definitions)
        {
            m_definitions.insert_or_assign(name, value);
        }
    }

    void Environment::Set(const std::string name, const Value value) noexcept
    {
        m_definitions.insert_or_assign(std::move(name), std::move(value));
    }
} // namespace lisp
