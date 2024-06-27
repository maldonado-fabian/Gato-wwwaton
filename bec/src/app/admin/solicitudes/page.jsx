import React from 'react'
import FlowNavAdmin from '../../../components/navbardamin'

const AdminDashboard = () => {
  return (
    <div>
            <FlowNavAdmin></FlowNavAdmin>

      {/* Main content */}
      <div className="container mx-auto mt-8">
        {/* Search bar */}
        <div className="mt-4">
          <label className="block text-gray-700">Buscar Código de documento:</label>
          <div className="flex items-center mt-2">
            <input 
              type="text" 
              className="w-full p-2 border border-gray-300 rounded-md" 
              placeholder="Ingrese código"
            />
            <button className="ml-2 px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600">
              Buscar
            </button>
          </div>
        </div>

        {/* Solicitudes */}
        <div className="mt-8">
          {/* Ejemplo de tarjeta de solicitud */}
          <div className="bg-gray-300 p-4 rounded-md shadow-md mb-4">
            <div className="flex">
              <div className="w-24 h-32 bg-gray-400 flex items-center justify-center text-white">
                Portada
              </div>
              <div className="ml-4 flex-1 text-black">
                <h2 className="text-lg font-bold">How to fight</h2>
                <p>Autor: Bruce Lee</p>
                <p>Formato: Libro</p>
                <p>Detalles de publicación: San Francisco, Honk Editions, 1973</p>
                <p>Disponibilidad: 1</p>
                <p>Código: hwr1342</p>
              </div>
              <div className="ml-auto text-black">
                <h3 className="font-semibold">Ubicación:</h3>
                <p>Estante: 5</p>
                <p>Fila: 12</p>
                <p>Columna: F</p>
                <p>Dewey: 863.62 Q217c</p>
              </div>
            </div>
          </div>
          
          {/* Repite el bloque anterior para más solicitudes */}
          <div className="bg-gray-300 p-4 rounded-md shadow-md mb-4">
            <div className="flex">
              <div className="w-24 h-32 bg-gray-400 flex items-center justify-center text-white">
                Portada
              </div>
              <div className="ml-4 flex-1 text-black">
                <h2 className="text-lg font-bold">How to rizz a leel 10 gyat</h2>
                <p>Autor: Vinicius</p>
                <p>Formato: Libro</p>
                <p>Detalles de publicación: Ohio, Book lords, 2014</p>
                <p>Disponibilidad: 1</p>
                <p>Código: hrl1342</p>
              </div>
              <div className="ml-auto text-black">
                <h3 className="font-semibold">Ubicación:</h3>
                <p>Estante: 7</p>
                <p>Fila: 3</p>
                <p>Columna: A</p>
                <p>Dewey: 823.62 Q217a</p>
              </div>
            </div>
          </div>
          
        </div>
      </div>
    </div>
  );
};

export default AdminDashboard;