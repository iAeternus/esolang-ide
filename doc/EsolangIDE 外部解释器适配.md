## EsolangIDE 外部解释器适配

### cpp 案例

```cpp
#include <bits/stdc++.h>

/**
 * @brief 解释器逻辑
 * @param tokens 代码行，每个元素都是一个代码token
 * @param input 输入流，使用 input >> a 来代替 std::cin >> a
 * @return 程序返回值，return 0以正常结束程序
 */
int interpreter(const std::vector<std::string> &tokens, std::ifstream &input) {
    // TODO 在这里写解释器逻辑
}

int main(int argc, char *argv[]) {
    if (argc < 3) {
        std::cerr << "Usage: " << argv[0] << " <code_file> <input_file>" << std::endl;
        return 1;
    }

    std::ifstream code_file(argv[1]);
    if (!code_file) {
        std::cerr << "Failed to open code file: " << argv[1] << std::endl;
        return 1;
    }

    std::vector<std::string> tokens;
    std::string token;
    while (code_file >> token) {
        tokens.push_back(std::move(token));
    }
    code_file.close();

    std::ifstream input_file(argv[2]);
    if (!input_file) {
        std::cerr << "Failed to open input file: " << argv[2] << std::endl;
        return 1;
    }

    std::ios_base::sync_with_stdio(false);
    std::cin.tie(nullptr);

    auto result = interpreter(tokens, input_file);

    input_file.close();
    return result;
}
```

**使用**

```shell
./interpreter <code_file> <input_file>
```

