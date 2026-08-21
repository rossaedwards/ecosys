// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title CasinoLogic
 * @dev Provably fair casino game logic
 */
contract CasinoLogic {
    struct GameResult {
        uint256 gameId;
        address player;
        uint256 bet;
        uint256 payout;
        bytes32 randomness;
        bool verified;
    }

    mapping(uint256 => GameResult) public games;
    uint256 public gameCounter;
    address public owner;

    event GamePlayed(
        uint256 indexed gameId,
        address indexed player,
        uint256 bet,
        uint256 payout,
        bytes32 randomness
    );

    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    function playGame(
        bytes32 randomness,
        uint256 bet
    ) external payable returns (uint256) {
        require(msg.value >= bet, "Insufficient payment");
        
        uint256 gameId = gameCounter++;
        
        // Simplified game logic - would implement actual game here
        uint256 payout = calculatePayout(bet, randomness);
        
        games[gameId] = GameResult({
            gameId: gameId,
            player: msg.sender,
            bet: bet,
            payout: payout,
            randomness: randomness,
            verified: true
        });

        if (payout > 0) {
            payable(msg.sender).transfer(payout);
        }

        emit GamePlayed(gameId, msg.sender, bet, payout, randomness);
        
        return payout;
    }

    function calculatePayout(
        uint256 bet,
        bytes32 randomness
    ) internal pure returns (uint256) {
        // Simplified payout calculation
        // In production, this would use the randomness to determine game outcome
        uint256 randomValue = uint256(randomness) % 100;
        
        if (randomValue < 48) { // 48% win chance (2% house edge)
            return bet * 2;
        }
        
        return 0;
    }

    function verifyGame(uint256 gameId) external view returns (bool) {
        GameResult memory game = games[gameId];
        return game.verified;
    }
}

