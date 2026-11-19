import { CarboTrack } from 'carbotrack';
import express from 'express';

const tracker = new CarboTrack({ cloud: 'aws', region: 'us-east-1', budget: 1000 });
const app = express();

app.use((req, res, next) => {
  const started = Date.now();
  res.on('finish', () => {
    tracker.trackApiCall({ url: req.originalUrl, bytesTransferred: Number(res.getHeader('content-length') || 0), duration: Date.now() - started });
  });
  next();
});

app.get('/', (_req, res) => res.json({ status: 'ok', emissions: tracker.getEmissions() }));
app.listen(3000, () => console.log('Example listening on http://127.0.0.1:3000'));
