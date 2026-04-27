use anchor_lang::prelude::*;

declare_id!("Agc7V4RopZxV1JnpyQWrLPtKSLjcDk5RMjgNQXHvk2Gc");

#[program]
pub mod gamevault {
    use super::*;

    // ================= CREAR COLECCION =================
    pub fn crear_coleccion(
        ctx: Context<NuevaColeccion>,
        nombre: String,
    ) -> Result<()> {

        let owner_id = ctx.accounts.owner.key();
        let juegos: Vec<Juego> = Vec::new();

        ctx.accounts.coleccion.set_inner(Coleccion {
            owner: owner_id,
            nombre,
            juegos,
        });

        Ok(())
    }

    // ================= AGREGAR JUEGO =================
    pub fn agregar_juego(
        ctx: Context<ModificarColeccion>,
        nombre: String,
        plataforma: String,
    ) -> Result<()> {

        require!(
            ctx.accounts.coleccion.owner == ctx.accounts.owner.key(),
            ErrorJuego::NoEresElOwner
        );

        let juego = Juego {
            nombre,
            plataforma,
            completado: false,
        };

        ctx.accounts.coleccion.juegos.push(juego);

        Ok(())
    }

    // ================= VER JUEGOS =================
    pub fn ver_juegos(ctx: Context<ModificarColeccion>) -> Result<()> {

        require!(
            ctx.accounts.coleccion.owner == ctx.accounts.owner.key(),
            ErrorJuego::NoEresElOwner
        );

        msg!("Lista actual de juegos: {:#?}", ctx.accounts.coleccion.juegos);

        Ok(())
    }

    // ================= ELIMINAR JUEGO =================
    pub fn eliminar_juego(
        ctx: Context<ModificarColeccion>,
        nombre: String,
    ) -> Result<()> {

        require!(
            ctx.accounts.coleccion.owner == ctx.accounts.owner.key(),
            ErrorJuego::NoEresElOwner
        );

        let juegos = &mut ctx.accounts.coleccion.juegos;

        for i in 0..juegos.len() {
            if juegos[i].nombre == nombre {
                juegos.remove(i);
                msg!("Juego eliminado correctamente");
                return Ok(());
            }
        }

        Err(ErrorJuego::JuegoNoExiste.into())
    }

    // ================= CAMBIAR ESTADO =================
    pub fn alternar_estado(
        ctx: Context<ModificarColeccion>,
        nombre: String,
    ) -> Result<()> {

        require!(
            ctx.accounts.coleccion.owner == ctx.accounts.owner.key(),
            ErrorJuego::NoEresElOwner
        );

        let juegos = &mut ctx.accounts.coleccion.juegos;

        for i in 0..juegos.len() {
            if juegos[i].nombre == nombre {
                let estado = juegos[i].completado;
                juegos[i].completado = !estado;

                msg!("Estado actualizado");
                return Ok(());
            }
        }

        Err(ErrorJuego::JuegoNoExiste.into())
    }
}

// ================= ERRORES =================
#[error_code]
pub enum ErrorJuego {
    #[msg("No eres el propietario de esta coleccion")]
    NoEresElOwner,

    #[msg("El juego no existe")]
    JuegoNoExiste,
}

// ================= CUENTA PRINCIPAL =================
#[account]
#[derive(InitSpace)]
pub struct Coleccion {

    owner: Pubkey,

    #[max_len(50)]
    nombre: String,

    #[max_len(10)]
    juegos: Vec<Juego>,
}

// ================= STRUCT JUEGO =================
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Juego {

    #[max_len(50)]
    nombre: String,

    #[max_len(30)]
    plataforma: String,

    completado: bool,
}

// ================= CONTEXTO CREAR =================
#[derive(Accounts)]
pub struct NuevaColeccion<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Coleccion::INIT_SPACE + 8,
        seeds = [b"gamevault", owner.key().as_ref()],
        bump
    )]
    pub coleccion: Account<'info, Coleccion>,

    pub system_program: Program<'info, System>,
}

// ================= CONTEXTO MODIFICAR =================
#[derive(Accounts)]
pub struct ModificarColeccion<'info> {

    pub owner: Signer<'info>,

    #[account(mut)]
    pub coleccion: Account<'info, Coleccion>,
}
