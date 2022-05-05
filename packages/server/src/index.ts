import {Game} from 'superjack';
import cors from 'cors';
import express from 'express';
import {join} from 'path';

const PORT = 3000;

const app = express();
app.use(cors());
app.use(express.json());

// Serve static resources from the "public" folder (ex: when there are images to display)
app.use(express.static(join(__dirname, '../../frontend/public')));

// Serve the HTML page
app.get('/', (req: any, res: any) => {
  res.sendFile(join(__dirname, '../../frontend/public', 'index.html'));
});

// API
const game = new Game();
app.get('/api', (req: any, res: any) => {
  res.status(200).json({num: game.num});
});
app.post('/api', (req: any, res: any) => {
  game.increment(req.body.num);
  res.status(200).json({num: game.num});
});

app.listen(PORT, () => {
  console.log(`Server listening at http://localhost:${PORT}`);
});
