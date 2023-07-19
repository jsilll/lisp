#pragma once

#include <string>
#include <optional>
#include <unordered_map>

namespace lisp
{
    class Value;

    class Environment final
    {
    public:
        [[nodiscard]] std::optional<Value> Get(const std::string &name) const noexcept;

        void SetParent(const Environment &parent) noexcept;

        void Combine(const Environment &other) noexcept;

        void Set(const std::string &name, const Value &value) noexcept;

    private:
        const Environment *m_parent{nullptr};
        std::unordered_map<std::string, Value> m_definitions;
    };
} // namespace lisp