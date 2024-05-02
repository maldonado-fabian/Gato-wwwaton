import "bootstrap/dist/css/bootstrap.min.css";
import { NavBar} from "../../components/NavBar";

const Home = () => {
    const libros = [
        {
          titulo: 'Naranja mecanica',
          descripcion: 'DIABLOOOOOO Q BUENA PELICULA',
          imagenUrl: 'https://m.media-amazon.com/images/I/414l8Mx6J3L._SL500_.jpg'
        },
        {
          titulo: 'Libro 2',
          descripcion: 'Descripción del libro 2',
          imagenUrl: 'https://www.penguinlibros.com/cl/1787435/crepusculo-saga-crepusculo-1.jpg'
        },
        {
            titulo: 'Floppa>>>>',
            descripcion: 'oh yeah',
            imagenUrl: 'https://dthezntil550i.cloudfront.net/46/latest/462109120440496000008176754/1280_960/4310c73c-9585-4d8b-8561-3b7fcb70df50.png'
        },
        // Agrega más libros aquí
      ];
  return (
    <div>
      <NavBar />
      <div className="jumbotron mt-4 p-5 bg-primary text-white rounded">
        <h1>Bienvenido a nuestra biblioteca</h1>
        <p>Explora nuestra colección de libros y descubre tu próxima gran lectura.</p>
      </div>
      <div className="container">
        <h2>Libros destacados</h2>
        <div className="row">
          {libros.map((libro, index) => (
            <div className="col-sm-4" key={index}>
              <div className="card">
                <img src={libro.imagenUrl} className="card-img-top" alt={libro.titulo} />
                <div className="card-body">
                  <h5 className="card-title">{libro.titulo}</h5>
                  <p className="card-text">{libro.descripcion}</p>
                  <a href="#" className="btn btn-primary">Ver más</a>
                </div>
              </div>
            </div>
          ))}
          {/* Este es solo un ejemplo, tendrías que reemplazarlo con tus propios datos */} 
        </div>
      </div>
    </div>
  );
}

export default Home;