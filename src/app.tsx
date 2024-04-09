import express, { Request, Response } from 'express';
const app = express();
const port = 3000;

//middleware
app.use(express.json());

app.get('/', (req, res) => {
  res.send('Hello World!');
});

app.post('/rating', (req: Request, res: Response) => {
  try {
    const { context, message } = req.body;
    const { domain, country, city, action, core_version, transaction_id } = context;
    const { ratings } = message;

    console.log('Received rating data:');
    console.log('Domain:', domain);
    console.log('Country:', country);
    console.log('City:', city);
    console.log('Action:', action);
    console.log('Core Version:', core_version);
    console.log('Transaction ID:', transaction_id);

    // todo : integerate rating with solana smart contract
    console.log('Ratings:', ratings);

    res.status(200).json({ success: true, message: 'Rating data received successfully.' });
  } catch (error) {
    console.error('Error processing rating request:', error);
    res.status(500).json({ success: false, message: 'An error occurred while processing the rating request.' });
  }
});

app.listen(port, () => {
  return console.log(`Express is listening at http://localhost:${port}`);
});