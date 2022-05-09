package robot_room_cleaner;

import java.util.*;

interface Robot {
	public boolean move();
	public void turnLeft();
	public void turnRight();
	public void clean();
}

class RobotImpl implements Robot {
	private int[][] map;
	private int[][] cellsCleaned;
	private int nbCellsToClean, nbCleanedCells;
	private int[] nextMovement; // (x,y)
	private int[] curPos;

	public RobotImpl(int[][] map, int[] curPos) {
		this.map = map;
		this.cellsCleaned = new int[map.length][map[0].length];
		this.nextMovement = new int[]{-1,0};
		this.curPos = curPos;

		this.nbCellsToClean = 0;
		this.nbCleanedCells = 0;
		for (int i=0; i<map.length; i++) {
			for (int j=0; j<map[0].length; j++) {
				cellsCleaned[i][j] = map[i][j];
				if (map[i][j] == 0)
					this.nbCellsToClean++;
			}
		}
	}

	@Override
	public boolean move() {
		System.out.println("curPos[0]="+curPos[0]+" curPos[1]="+curPos[1]);
		//System.out.println("nextMovement[0]="+nextMovement[0]+" nextMovement[1]="+nextMovement[1]);

		if (!nextMoveInBound())
			return false;
		
		int nextY = curPos[0] + nextMovement[0];
		int nextX = curPos[1] + nextMovement[1];

		if (map[nextY][nextX] == 1)
			return false;
		
		curPos[0] = nextY;
		curPos[1] = nextX;
		return true;
	}

	@Override
	public void turnLeft() {
		if (nextMovement[0] == -1) { // Facing up
			nextMovement[1] = -1;
			nextMovement[0] = 0;
		}
		else if (nextMovement[0] == 1) { // Facing down
			nextMovement[1] = 1;
			nextMovement[0] = 0;
		}
		else if (nextMovement[1] == -1) { // Facing left
			nextMovement[1] = 0;
			nextMovement[0] = 1;
		}
		else { // Facing right
			nextMovement[1] = 0;
			nextMovement[0] = -1;
		}

	}

	@Override
	public void turnRight() {
		turnLeft();
		turnLeft();
		turnLeft();
	}

	@Override
	public void clean() {
		if (cellsCleaned[curPos[0]][curPos[1]] == 0) {
			cellsCleaned[curPos[0]][curPos[1]] = 1;
			nbCleanedCells++;
		}
		if (allCleaned()) {
			System.out.println("All cells has been cleaned!");
		}
	}

	private boolean nextMoveInBound() {
		int nextY = curPos[0] + nextMovement[0];
		int nextX = curPos[1] + nextMovement[1];
		
		return !(nextY < 0 || nextY >= map.length ||
			nextX < 0 || nextX >= map[0].length);
	}

	private boolean allCleaned() {
		return nbCellsToClean == nbCleanedCells;
	}
}

class Cell {
	int row, col;

	public Cell(int row, int col) {
		this.row = row;
		this.col = col;
	}

	@Override
	public boolean equals(Object o) {
		if (o instanceof Cell) {
			Cell c = (Cell)o;
			return row == c.row && col == c.col;
		}
		return false;
	}

	@Override
	public int hashCode() {
		return row ^ col;
	}
}

enum Direction { UP, LEFT, DOWN, RIGHT };

class Solution {
	Robot robot;
	Cell curPos;
	Direction curDirection;
	Hashtable<Cell, boolean[]> cells = new Hashtable<>(); // boolean[i] is true when boolean[Direction.ordinal()] has been visited
	Stack<Cell> path = new Stack<>();

    public void cleanRoom(Robot robot) {
		this.robot = robot;
		curDirection = Direction.UP;
		curPos = new Cell(0,0);

		path.push(curPos);
		cells.put(curPos, new boolean[]{false, false, false, false});
		while (!path.isEmpty()) {
			curPos = path.peek();

			Cell neighbor = selectNextCell();
			if (neighbor == null) {
				path.pop();
				if (!path.isEmpty())
					goToCell(findDirection(path.peek()));
				continue;
			}

			Direction nextDirection = findDirection(neighbor);
			cells.get(curPos)[nextDirection.ordinal()] = true;
			cells.get(neighbor)[getOppositeDirection(nextDirection).ordinal()] = true;
			if (goToCell(nextDirection))
				path.push(neighbor);
		}
    }

	private Cell selectNextCell() {
		boolean[] exploredDirection = cells.get(curPos);
		for (int i=0; i<exploredDirection.length; i++) {
			if (!exploredDirection[i]) {
				Cell neighbor = getCell(Direction.values()[i]);
				for (boolean b : cells.get(neighbor)) {
					if (!b)
						return neighbor;
				}
				exploredDirection[i] = true;
			}
		}
		return null;
	}

	// Return the cell neighbor to curPos, create a new entry in cells is key does not exist
	private Cell getCell(Direction direction) {
		Cell neighbor;
		switch (direction) {
			case UP:
				neighbor = new Cell(curPos.row-1, curPos.col);
				break;
			case LEFT:
				neighbor = new Cell(curPos.row, curPos.col-1);
				break;
			case DOWN:
				neighbor = new Cell(curPos.row+1, curPos.col);
				break;
			default:
				neighbor = new Cell(curPos.row, curPos.col+1);
		}
		if (!cells.containsKey(neighbor))
			cells.put(neighbor, new boolean[]{false, false, false, false});

		return neighbor;
	}

	// Output where the destination cell is regarding curPos
	private Direction findDirection(Cell dest) {
		int diffRow = curPos.row - dest.row;
		int diffCol = curPos.col - dest.col;

		if (diffCol == -1)
			return Direction.RIGHT;
		if (diffCol == 1)
			return Direction.LEFT;
		if (diffRow == -1)
			return Direction.DOWN;

		return Direction.UP;
	}

	// Rotate the robot to align with destDirection
	private void alignDirection(Direction destDirection) {
		int directionDiff = curDirection.ordinal() - destDirection.ordinal();

		if (directionDiff < 0) {
			for (int i=0; i<Math.abs(directionDiff); i++)
				robot.turnRight();
		}
		else {
			for (int i=0; i<directionDiff; i++)
				robot.turnLeft();
		}
		curDirection = destDirection;
	}

	private Direction getOppositeDirection(Direction direction) {
		switch (direction) {
			case UP:
				return Direction.DOWN;
			case LEFT:
				return Direction.RIGHT;
			case DOWN:
				return Direction.UP;
			default:
				return Direction.LEFT;
		}
	}

	// Try to go to destination cell
	private boolean goToCell(Direction destDirection) {
		alignDirection(destDirection);
		robot.clean();
		return robot.move();
	}

}

class Main {
	public static void main(String args[]) {
		System.out.println("A cleaning robot AI!");
		Solution solution = new Solution();

		int[][] map = {
			{0,0,0},
			{0,0,0},
			{0,0,0}
		};
		int[] curPos = {1,1};
		
		Robot robot = new RobotImpl(map, curPos);
		solution.cleanRoom(robot);
	}
}
