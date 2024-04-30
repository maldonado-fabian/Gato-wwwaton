import "bootstrap/dist/css/bootstrap.min.css";
import { NavBar} from "../../components/navbar";

const login = () => {

  return (
      <div class="container-fluid">
        
        <NavBar />

        <div class="container-sm p-5">
          <div class="row justify-content-center align-items-center">
              <div class="col-10 text-center">
                  <p class="fw-bolder fs-2">Iniciar sesión</p>
              </div>
              <div class="col-10 col-sm-6 col-md-4 col-lg-3">
                <form>
                  <div class="mb-3">
                    <label htmlFor="rut" class="form-label text-body-secondary">Rut</label>
                    <br />
                    <input
                      type="text"
                      id="rut"
                      aria-describedby="rutHelp"
                    />
                  </div>
                  <div class="mb-3">
                    <label htmlFor="pass" class="form-label text-body-secondary">Contraseña</label>
                    <br />
                    <input
                      type="password"
                      id="pass"
                      aria-describedby="passHelp"
                    />
                  </div>
                  <div class="d-grip gap-2 d-md-block">
                    <button type="button" class="btn btn-secondary text-center form-control">Ingresar</button>
                  </div>
                </form>
              </div>
          </div>
        </div>
      </div>
  );
};

export default login;
