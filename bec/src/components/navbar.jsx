"use client";
import { Button, Navbar } from "flowbite-react";

export default function FlowNav() {
  return <>
    <div className="bg-[#ABBEB3]">
      <div className="container mx-auto">
        <Navbar fluid rounded className="bg-[#ABBEB3]">
          <Navbar.Brand href="/">
            <img class="w-12 h-12 rounded-full mr-2" src="https://i.postimg.cc/7Zgqf77m/6d09693e-89cf-4f76-be1f-ba16cbc1c2df.jpg" alt="logo" />
            <span className="self-center text-[#09331F] whitespace-nowrap text-xl font-semibold dark:text-white">Biblioteca Estación Central</span>
          </Navbar.Brand>
          <div className="flex md:order-2">
            <Button pill color="success" className="mr-[2rem] " size="lg" href="/login">Login</Button>
            <Button pill color="success" size="lg" href="/register">Register</Button>
            <Navbar.Toggle />
          </div>
          <Navbar.Collapse className="bg-[#779481] text-[#09331F]  p-5 px-10 rounded-full">
            <Navbar.Link href="/" className=" text-[#09331F] text-xl ">
              Inicio
            </Navbar.Link>
            <Navbar.Link href="/catalogo" className="mx-10 text-[#09331F] text-xl ">Catalogo</Navbar.Link>
            <Navbar.Link href="#" className=" text-[#09331F] text-xl ">FAQ</Navbar.Link>
          </Navbar.Collapse>
        </Navbar>
      </div>
    </div>
  </>

}
