"use client";
import React, {useState} from 'react'
import { Navbar, Modal, Button, TextInput, Label, Select, Checkbox  } from 'flowbite-react';

export default function FlowNavAdmin({show, onClose, onClick }) {
    const [showModal, setShowModal] = useState(false);
    const handleShow = () => setShowModal(true);
    const handleClose = () => setShowModal(false);
    // Estado para los datos del formulario
    const [formData, setFormData] = useState({
        tipo: '',
        titulo: '',
        autor: '',
        editorial: '',
        ano: '',
        edicion: '',
        categoria: '',
        isbn: '',
        libros: [{ ubicacion: '', disponibilidad: true }]
    });

      // Función para manejar cambios en los campos del formulario
    const handleChange = (e) => {
        const { name, value } = e.target;
        setFormData(prevData => ({
        ...prevData,
        [name]: value
        }));
    };

      // Función para manejar el envío del formulario
    const handleSubmit = (e) => {
        e.preventDefault();
        console.log("Datos del formulario:", formData);
        // Aquí puedes hacer algo con los datos, como enviarlos a una API
        handleClose();
    };

      // Función para manejar el cambio en los campos de "libros"
    const handleLibrosChange = (index, field, value) => {
        const updatedLibros = formData.libros.map((libro, i) => (
        i === index ? { ...libro, [field]: value } : libro
        ));
        setFormData(prevData => ({
        ...prevData,
        libros: updatedLibros
        }));
    };

    // Función para agregar una nueva entrada en "libros"
    const handleAddLibro = () => {
        setFormData(prevData => ({
        ...prevData,
        libros: [...prevData.libros, { ubicacion: '', disponibilidad: true }]
        }));
    };

    // Función para eliminar una entrada en "libros"
    const handleRemoveLibro = (index) => {
        setFormData(prevData => ({
        ...prevData,
        libros: prevData.libros.filter((_, i) => i !== index)
        }));
    };

    return <>
    <div className="bg-[#ABBEB3]">
      <div className="container mx-auto">
        <Navbar fluid rounded className="bg-[#ABBEB3]">
          <Navbar.Brand href="/">
            <img class="w-12 h-12 rounded-full mr-2" src="https://i.postimg.cc/7Zgqf77m/6d09693e-89cf-4f76-be1f-ba16cbc1c2df.jpg" alt="logo" />
            <span className="self-center text-[#09331F] whitespace-nowrap text-xl font-semibold dark:text-white">Biblioteca Estación Central</span>
            </Navbar.Brand>
            <Navbar.Toggle />
            <div className="flex md:order-2">
            <Navbar.Collapse>
                <Button pill color="success" className="mr-[2rem] " size="lg" href='/admin'>Inicio</Button>
                <Button onClick={handleShow} pill color="success" className="mr-[2rem] " size="lg">Añadir Documento</Button>
                <Button pill color="success" className="mr-[2rem] " size="lg">Eliminar Documento</Button>
                <Button pill color="success" className="mr-[2rem] " size="lg" href='/admin/solicitudes'>Solicitudes</Button>
                <Button pill color="success" className="mr-[2rem] " size="lg" href='/admin/prestamos'>Prestamos</Button>
            </Navbar.Collapse>
          </div>
        </Navbar>
      {/* Modal con el formulario */}
      <Modal
        show={showModal}
        onClose={handleClose}
      >
        <Modal.Header>
          Agregar Documento
        </Modal.Header>
        <Modal.Body>
          <form onSubmit={handleSubmit}>
            {/* Campo para el tipo */}
            <div className="mb-4">
              <Label htmlFor="tipo" value="Tipo de Documento" />
              <Select id="tipo" name="tipo" onChange={handleChange} required>
                <option value="">Seleccione el tipo</option>
                <option value="Libro">Libro</option>
                <option value="Revista">Revista</option>
                <option value="Artículo">Artículo</option>
              </Select>
            </div>

            {/* Campo para el título */}
            <div className="mb-4">
              <Label htmlFor="titulo" value="Título" />
              <TextInput
                id="titulo"
                name="titulo"
                type="text"
                onChange={handleChange}
                required
              />
            </div>

            {/* Campo para el autor */}
            <div className="mb-4">
              <Label htmlFor="autor" value="Autor" />
              <TextInput
                id="autor"
                name="autor"
                type="text"
                onChange={handleChange}
                required
              />
            </div>

            {/* Campo para la editorial */}
            <div className="mb-4">
              <Label htmlFor="editorial" value="Editorial" />
              <TextInput
                id="editorial"
                name="editorial"
                type="text"
                onChange={handleChange}
                required
              />
            </div>

            {/* Campo para el año */}
            <div className="mb-4">
              <Label htmlFor="ano" value="Año" />
              <TextInput
                id="ano"
                name="ano"
                type="number"
                onChange={handleChange}
                required
              />
            </div>

            {/* Campo para la edición */}
            <div className="mb-4">
              <Label htmlFor="edicion" value="Edición" />
              <TextInput
                id="edicion"
                name="edicion"
                type="text"
                onChange={handleChange}
                required
              />
            </div>

            {/* Campo para la categoría */}
            <div className="mb-4">
              <Label htmlFor="categoria" value="Categoría" />
              <TextInput
                id="categoria"
                name="categoria"
                type="text"
                onChange={handleChange}
                required
              />
            </div>

            {/* Campo para el ISBN */}
            <div className="mb-4">
              <Label htmlFor="isbn" value="ISBN" />
              <TextInput
                id="isbn"
                name="isbn"
                type="text"
                onChange={handleChange}
                required
              />
            </div>
            {/* Campos dinámicos para los libros */}
            <div className="mb-4">
              <Label value="Ubicaciones" />
              {formData.libros.map((libro, index) => (
                <div key={index} className="space-y-2 mb-2">
                  <div className="flex space-x-4">
                    <TextInput
                      id={`ubicacion-${index}`}
                      name={`ubicacion-${index}`}
                      type="text"
                      placeholder={`Ubicación ${index + 1}`}
                      value={libro.ubicacion}
                      onChange={(e) => handleLibrosChange(index, 'ubicacion', e.target.value)}
                      required
                    />
                    <div className="flex items-center">
                      <Checkbox
                        id={`disponibilidad-${index}`}
                        name={`disponibilidad-${index}`}
                        checked={libro.disponibilidad}
                        onChange={(e) => handleLibrosChange(index, 'disponibilidad', e.target.checked)}
                      />
                      <Label htmlFor={`disponibilidad-${index}`} className="ml-2">Disponibilidad</Label>
                    </div>
                    <Button
                      color="red"
                      size="small"
                      onClick={() => handleRemoveLibro(index)}
                    >
                      Eliminar
                    </Button>
                  </div>
                </div>
              ))}
              <Button
                color="gray"
                size="small"
                className="mt-2"
                onClick={handleAddLibro}
              >
                Añadir Libro
              </Button>
            </div>            
            <Button type="submit" className="mt-4">
              Guardar
            </Button>
          </form>
        </Modal.Body>
      </Modal>
      </div>
    </div>
  </>

}