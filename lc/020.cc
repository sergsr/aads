class Solution {
 public:
  bool isValid(const string &s) {
    enum Bracket { Round, Square, Curly };
    stack<Bracket> parens;
    for (char c : s) {
      switch (c) {
        case '(':
          parens.push(Round);
          break;
        case '[':
          parens.push(Square);
          break;
        case '{':
          parens.push(Curly);
          break;
        case ')':
          if (parens.empty() || Round != parens.top()) return false;
          parens.pop();
          break;
        case ']':
          if (parens.empty() || Square != parens.top()) return false;
          parens.pop();
          break;
        case '}':
          if (parens.empty() || Curly != parens.top()) return false;
          parens.pop();
          break;
        default:
          return false;
      }
    }
    return parens.empty();
  }
};
