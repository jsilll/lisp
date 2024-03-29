#pragma once

#include <optional>
#include <string>
#include <unordered_map>

namespace lisp {
class Value;

class Environment final {
  public:
    [[nodiscard]] ::std::optional<Value>
    Get(const ::std::string &name) const noexcept;

    void SetParent(const Environment &parent) noexcept { m_parent = &parent; }

    void Combine(const Environment &other) noexcept;

    void Set(::std::string name, Value value) noexcept;

  private:
    const Environment *m_parent{nullptr};
    ::std::unordered_map<::std::string, Value> m_definitions;
};
}   // namespace lisp