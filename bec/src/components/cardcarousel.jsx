import React from 'react';
import { Card, CardDeck } from 'flowbite-react';
impo
const CardCarousel = () => {
  return (
    <div className="container mx-auto mt-5">
      <CardDeck>
        <Card>
          <Card.Body>
            <Card.Title>Card 1</Card.Title>
            <Card.Text>
              Some quick example text to build on the card title and make up the bulk of the card's content.
            </Card.Text>
          </Card.Body>
        </Card>
        <Card>
          <Card.Body>
            <Card.Title>Card 2</Card.Title>
            <Card.Text>
              Some quick example text to build on the card title and make up the bulk of the card's content.
            </Card.Text>
          </Card.Body>
        </Card>
        {/* Agrega más cartas según sea necesario */}
      </CardDeck>
    </div>
  );
};

export default CardCarousel;