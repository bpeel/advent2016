#include <iostream>
#include <cstdlib>
#include <memory>
#include <deque>
#include <vector>
#include <set>
#include <utility>
#include <limits>

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
        int depth;
};

SearchNode::SearchNode(int x, int y)
        : parent(0),
          x(x), y(y),
          lastDirection(0),
          depth(0)
{
}

SearchNode::SearchNode(const std::shared_ptr<SearchNode> &parent,
                       int x, int y,
                       int lastDirection)
        : parent(parent),
          x(x), y(y),
          lastDirection(lastDirection),
          depth(parent->depth + 1)
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

std::pair<std::shared_ptr<SearchNode>, int>
solve(const Maze &maze,
      int goalX,
      int goalY,
      int limit = std::numeric_limits<int>::max())
{
        std::deque<std::shared_ptr<SearchNode>> queue;
        std::set<std::pair<int, int>> history;

        queue.push_back(std::make_shared<SearchNode>(1, 1));
        history.insert(std::make_pair(1, 1));

        while (!queue.empty()) {
                std::shared_ptr<SearchNode> node(queue.front());

                queue.pop_front();

                if (node->x == goalX && node->y == goalY)
                        return std::make_pair(node, history.size());

                if (node->depth >= limit)
                        continue;

                for (int direction = 0; direction < 4; direction++) {
                        int x, y;

                        move(direction, node->x, node->y, x, y);

                        if (maze.isWall(x, y))
                                continue;

                        if (!history.insert(std::make_pair(x, y)).second)
                                continue;

                        auto newNode = std::make_shared<SearchNode>(node,
                                                                    x, y,
                                                                    direction);
                        queue.push_back(newNode);
                }
        }

        return std::make_pair(std::shared_ptr<SearchNode>(0), history.size());
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

        std::shared_ptr<SearchNode> part1(solve(maze, GOAL_X, GOAL_Y).first);

        std::cout << "Part 1" << std::endl;

        if (part1)
                printSolution(part1.get());
        else
                std::cout << "No solution found" << std::endl;

        int part2(solve(maze, -1, -1, 50).second);

        std::cout << "Part 2: " << part2 << std::endl;

        return 0;
}
