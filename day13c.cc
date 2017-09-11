#include <iostream>
#include <cstdlib>
#include <memory>
#include <deque>
#include <vector>

const int GOAL_X = 31;
const int GOAL_Y = 39;

class Maze {
private:
        int favouriteNumber;

public:
        explicit Maze(int favouriteNumber);

        bool isWall(int x, int y) const;
};

Maze::Maze(int favouriteNumber)
        : favouriteNumber(favouriteNumber)
{
}

bool
Maze::isWall(int x, int y) const
{
        if (x < 0 || y < 0)
                return true;

        int code = x * x + 3 * x + 2 * x * y + y + y * y + favouriteNumber;
        int bits = __builtin_popcount(code);

        return (bits & 1) == 1;
}

struct SearchNode {
        SearchNode(int x, int y);
        SearchNode(const std::shared_ptr<SearchNode> &parent,
                   int x, int y,
                   int lastDirection);

        std::shared_ptr<SearchNode> parent;
        int x, y;
        int lastDirection;
};

SearchNode::SearchNode(int x, int y)
        : parent(0),
          x(x), y(y),
          lastDirection(0)
{
}

SearchNode::SearchNode(const std::shared_ptr<SearchNode> &parent,
                       int x, int y,
                       int lastDirection)
        : parent(parent),
          x(x), y(y),
          lastDirection(lastDirection)
{
}

void
move(int direction,
     int startX, int startY,
     int &endX, int &endY)
{
        switch (direction) {
        default:
        case 0:
                endX = startX;
                endY = startY - 1;
                break;
        case 1:
                endX = startX;
                endY = startY + 1;
                break;
        case 2:
                endX = startX - 1;
                endY = startY;
                break;
        case 3:
                endX = startX + 1;
                endY = startY;
                break;
        }
}

std::shared_ptr<SearchNode>
solve(const Maze &maze)
{
        std::deque<std::shared_ptr<SearchNode>> queue;

        queue.push_back(std::make_shared<SearchNode>(1, 1));

        while (!queue.empty()) {
                std::shared_ptr<SearchNode> node(queue.front());

                queue.pop_front();

                if (node->x == GOAL_X && node->y == GOAL_Y)
                        return node;

                if (maze.isWall(node->x, node->y))
                        continue;

                for (int direction = 0; direction < 4; direction++) {
                        int x, y;

                        move(direction, node->x, node->y, x, y);

                        auto newNode = std::make_shared<SearchNode>(node,
                                                                    x, y,
                                                                    direction);
                        queue.push_back(newNode);
                }
        }

        return 0;
}

void
printSolution(const SearchNode *node)
{
        static const char *directionNames = "UDLR";
        std::vector<int> directions;

        while (node->parent) {
                directions.push_back(node->lastDirection);
                node = node->parent.get();
        }

        for (auto it = directions.begin(); it != directions.end(); ++it) {
                std::cout << directionNames[*it];
        }

        std::cout << std::endl;

        std::cout << directions.size() << std::endl;
}

int
main (int argc, char **argv)
{
        int favouriteNumber = 1350;

        if (argc > 1)
                favouriteNumber = std::strtoul(argv[1], 0, 0);

        Maze maze(favouriteNumber);

        for (int y = 0; y < 10; y++) {
                for (int x = 0; x < 10; x++) {
                        std::cout << (maze.isWall(x, y) ? '#' : ' ');
                }

                std::cout << std::endl;
        }

        std::shared_ptr<SearchNode> node(solve(maze));

        if (node)
                printSolution(node.get());
        else
                std::cerr << "No solution found" << std::endl;

        return 0;
}
