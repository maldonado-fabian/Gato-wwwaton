"use client";

import { Sidebar } from "flowbite-react";
import { BiBuoy } from "react-icons/bi";
import { HiArrowSmRight, HiChartPie, HiInbox, HiShoppingBag, HiTable, HiUser, HiViewBoards } from "react-icons/hi";

export function SideBar() {
  return (
<Sidebar  aria-label="Sidebar with content separator example" style={{ width: '400px' }}className="bg-black-500/75">
  <Sidebar.Items>
    <Sidebar.ItemGroup>
      <Sidebar.Item href="#" >
        Artes
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Biografías, Literatura Y Estudios Literarios
      </Sidebar.Item>

      <Sidebar.Item href="#">
        Computación Y Tecnología De La Información
      </Sidebar.Item>

      <Sidebar.Item href="#">
        Deportes Y Actividades De Ocio Al Aire Libre
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Derecho
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Economía, Finanzas, Empresa Y Gestión
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Estilos De Vida, Aficiones Y Ocio
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Ficción Y Temas Afines
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Filosofía Y Religión
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Historia Y Arqueología
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Infantiles, Juveniles Y Didácticos
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Lenguaje Y Lingüística
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Matemáticas Y Ciencias
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Medicina, Enfermería, Veterinaria
      </Sidebar.Item>

      <Sidebar.Item href="#">
        Salud, Relaciones Y Desarrollo Personal
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Sociedad Y Ciencias Sociales
      </Sidebar.Item>

      <Sidebar.Item href="#">
        Calificadores De Período De Tiempo
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Calificadores De Lengua
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Calificadores De Lugar
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Calificadores De Interés
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Calificadores De Fines Educativos
      </Sidebar.Item>
      <Sidebar.Item href="#">
        Calificadores De Estilo
      </Sidebar.Item>
    </Sidebar.ItemGroup>
  </Sidebar.Items>
</Sidebar>

  );
}
