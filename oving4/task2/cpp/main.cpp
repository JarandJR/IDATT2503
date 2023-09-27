#include <iostream>
#include <string>
#include <utility>

using namespace std;

class StringManipulator {
public:
    explicit StringManipulator(std::string  input) : input(std::move(input)) {}

    StringManipulator& replace(char target, const std::string& replacement) {
        size_t pos = 0;
        while ((pos = input.find(target, pos)) != std::string::npos) {
            input.replace(pos, 1, replacement);
            pos += replacement.length();
        }
        return *this;
    }

    string convert() {
        return replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .getInput();
    }

    string getInput() const {
        return input;
    }

private:
    std::string input;
};

int main() {
    StringManipulator manipulator("This is a <test> & example.");
    cout << manipulator.convert()<< endl;
}
