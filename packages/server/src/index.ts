// import {Game} from 'superjack';
import cors from 'cors';
import express from 'express';
import {Request, Response} from 'express';
import {join} from 'path';

const PORT = 3000;

const app = express();
app.use(cors());
app.use(express.json());

// Serve static resources from the "public" folder (ex: when there are images to display)
app.use(express.static(join(__dirname, '../../frontend/public')));

// Serve the HTML page
app.get('/', (req: Request, res: Response) => {
  res.sendFile(join(__dirname, '../../frontend/public', 'index.html'));
});

// API
app.get('/api', (req: Request, res: Response) => {
  res.status(200).json({
    /*num: game.num*/
  });
});
app.post('/api', (req: Request, res: Response) => {
  // game.increment(req.body.num);
  res.status(200).json({
    /*num: game.num*/
  });
});

app.listen(PORT, () => {
  console.log(`Server listening at http://localhost:${PORT}`);
});
