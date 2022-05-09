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

class Solution {
    public void cleanRoom(Robot robot) {
		findTopLeftCorner(robot);
		doUpDownCleaning(robot);
    }

	// Return robot facing down
	void findTopLeftCorner(Robot robot) {
		// Assume robot is facing up
		do {
			robot.clean();
		} while(robot.move());
		robot.turnLeft();
		do {
			robot.clean();
		} while(robot.move());
		robot.turnLeft();
	}

	void doUpDownCleaning(Robot robot) {
		// Assume robot is facing down
		while (true) {
			// Go down
			do {
				robot.clean();
			} while(robot.move());
			// Go one time right
			robot.turnLeft();
			if (!robot.move())
				break;
			// Face up
			robot.turnLeft();
			do {
				robot.clean();
			} while(robot.move());
			// Go on time right
			robot.turnRight();
			if (!robot.move())
				break;
			// Face down
			robot.turnRight();
		}
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
