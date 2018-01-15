% SafeFlight EKF derivation
% Written by Martin Deegan
clc

helper_functions;

% 19 state EKF
% Position: latitude, longitude, altitude
% Velocity: North, East, Vertical
% Attitude: quaternion representation
% Biases: Gyroscope, Accelerometer
% Magnetic Field
syms lat lon alt vn ve vz qx qy qz qw gbx gby gbz abx aby abz mx my mz
syms dlat dlon dalt dvn dve dvz dtx dty dtz dgbx dgby dgbz dabx daby dabz dmx dmy dmz
syms thrust
syms dt
syms x
syms dx
x = [lat; lon; alt; vn; ve; vz; qx; qy; qz; qw; gbx; gby; gbz; abx; aby; abz; mx; my; mz];
dx = [lat lon alt vn ve vz qx qy qz qw gbx gby gbz abx aby abz mx my mz];

% Control Variables
syms gxp gyp gzp axp ayp azp
syms gx gy gz ax ay az
syms up un
up = [gxp; gyp; gzp; axp; ayp; azp];
un = [gx; gy; gz; ax; ay; az];

% transition functions
syms remove_bias(r,b)
remove_bias(r, b) = r-b;
syms vec_mean(v1, v2)
vec_mean(v1, v2) = (v1+v2)/2;
syms predict_state(p, v, q, gb, ab, gp, ap, g, a, dt)
syms predict_state_corrected(p, v, q, gp, g, a, dt)
syms predict_state_velocity(qx, qy, qz, qw, vn, ve, vz, ax, ay, az, dt)
syms predict_state_attitude(qx, qy, qz, qw, gxp, gyp, gzp, gx, gy, gz, dt)
syms predict_state_position(lat, lon, alt, vn, ve, vz, dt)

dXddx = jacobian(add_e_state([lat; lon; alt; vn; ve; vz; qx; qy; qz; qw;...
                                gbx; gby; gbz; abx; aby; abz; mx; my; mz], ...
                                [dlat; dlon; dalt; dvn; dve; dvz; dtx; dty; dtz; ...
                                dgbx; dgby; dgbz; dabx; daby; dabz; dmx; dmy; dmz]), ...
                                [dlat dlon dalt dvn dve dvz dtx dty dtz dgbx dgby dgbz dabx daby dabz dmx dmy dmz]);
                            
A = jacobian(predict_state(x, up, un, dt), dx);
A(7:10,7:13) = 0;

syms fx fy fz
H_field = jacobian(rotate_field(x, [fx; fy; fz], thrust), dx);
V_field = jacobian(rotate_field(x, [fx; fy; fz], thrust), [fx fy fz]);

disp("Jacobian of the state with respect to the error state (X_dx):");


disp("Linearization of the state transition function (A):");
% disp(dXddx);
disp(H_field);
disp(V_field);

% --------- Prediction Step ----------------
function integrate_gyroscope_f = integrate_gyroscope(q, gp, gn, dt)
    w = (gp + gn)/2;
    omega_mean = omega_matrix(w);
    omega_prev = omega_matrix(gp);
    omega_next = omega_matrix(gn);
    expon = expm(omega_mean);
    second_term = (1/48)*(omega_next*omega_prev-omega_prev*omega_next)*dt^2;
    integrate_gyroscope_f = (expon + second_term)*q;
end

function predict_velocity_f = predict_velocity(v, a, q, dt)
    rot = rot_matrix(q);
    a_w = rot*a;
    v_p = v + a_w * dt;
    predict_velocity_f = v_p;
end

function predict_position_f = predict_position(p, v, dt)
    p_p = p + v*dt;
    predict_position_f = p_p;
end

function predict_state_f = predict_state(x, up, un, dt)
    un = un - x(11:16);
    q = integrate_gyroscope(x(7:10), up(1:3), un(1:3), dt);
    v = predict_velocity(x(4:6), un(4:6), x(7:10), dt);
    p = predict_position(x(1:3), x(4:6), dt);
    predict_state_f = [p; v; q; x(11:19)];
end

% -------------------- Updates -------------------------

function rotate_field_f = rotate_field(x, f, thr) 
    rot = rot_matrix(x(7:10));
    rotate_field_f = rot * (f  - [0; 0; thr]);
end


% -------------------------------Helper functions and variables --------------------------------------
                   

% Helper Matrices

function rot_matrix_f = rot_matrix(q)
rot_matrix_f = [1-2*q(2)^2-2*q(3)^2 2*q(1)*q(2)-2*q(3)*q(4) 2*q(1)*q(3)+2*q(2)*q(4);
               2*q(1)*q(2)+2*q(3)*q(4) 1-2*q(1)^2-2*q(3)^2 2*q(2)*q(3)-2*q(1)*q(4);
               2*q(1)*q(3)-2*q(2)*q(4) 2*q(2)*q(3)+2*q(1)*q(4) 1-2*q(1)^2-2*q(2)^2];
end

function q_prod_l_f = q_prod_l(q)
    q_prod_l_f = [q(4) q(3) -q(2) q(1); -q(3) q(4) q(1) q(2); q(2) -q(1) q(4) q(3); -q(1) -q(2) -q(3) q(4)];
end

function q_prod_r_f = q_prod_r(q)
    q_prod_r_f = [q(4) -q(3) q(2) q(1); q(3) q(4) -q(1) q(2); -q(2) q(1) q(4) q(3); -q(1) -q(2) -q(3) q(4)];
end

function skew_matrix_f = skew_matrix(w)
    skew_matrix_f = [0 -w(3) w(2); w(3) 0 -w(1); -w(2) w(1) 0];
end

function omega_matrix_f = omega_matrix(w)
    omega_matrix_f = [0 w(3) -w(2) w(1); -w(3) 0 w(1) w(2); w(2) -w(1) 0 w(3); -w(1) -w(2) -w(3) 0];
end

function small_e_to_dq_f = small_e_to_dq(theta)
    small_e_to_dq_f = [0.5*theta(1); 0.5*theta(2); 0.5*theta(3); 1];
end

function add_e_state_f = add_e_state(x, dx)
    p_prod = q_prod_r(x(7:10));
    dq = small_e_to_dq(dx(7:9));
    q_fixed = p_prod * dq;
    dx_dim = [dx(1:6); 0; 0; 0; 0; dx(10:18)];
    x_fixed = x + dx_dim;
    x_fixed(7:10) = q_fixed;
    add_e_state_f = x_fixed;
end
