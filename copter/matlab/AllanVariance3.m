% from import data
gx=gyro.VarName1;
gy=gyro.VarName2;
gy=gyro.VarName3;
t=cumsum(gyro.VarName4);


figure(2)
plot(t,gx)
%%
global N
global tau0

N=length(t);
tau0=mean(gyro.VarName4);

N=1e6;
gx2=gx(1:N);
NN=100;
Mlist=floor(logspace(0,log10(N),NN));
period=zeros(NN,1);
AOAV=zeros(NN,1);
for i=1:NN
    M=Mlist(i);
%     AAV(M)=AngleAllanVariance(gx,M);
    [period(i),AOAV(i)]=AverageOutputAllanVariance(gx2,M);
    display([num2str(i/NN*100),'%'])
end

%%
figure(1),hold off
loglog(period,AOAV,'o')

hold on
plot(period,1./period*1e-3,'r')
plot(period,1./sqrt(period)*1e-3,'g')
xlabel('Period (s)')
ylabel('radians per sec')

%%

%%

% function AAV=AngleAllanVariance(Omega,M)
% global tau0
% global N
% tau=M*tau0;
% theta=cumsum(Omega)*tau0;
% dtheta=theta(M:N)-theta(1:(N-M+1));
% AAV=1/2/tau^2/(N-2*M)*sum((dtheta(2:end)-dtheta(1:end-1)).^2);
% end

function [tau,AOAV]=AverageOutputAllanVariance(Omega,M)
global tau0
global N
tau=M*tau0;
kern=ones(M,1)/M;
OmegaK=conv(Omega,kern,'valid');
dOmega=OmegaK(1:(N-2*M+1))-OmegaK((1+M):end);

AOAV=1/2/(N-2*M)*sum(dOmega.^2);
end
